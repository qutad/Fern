use std::collections::HashMap;

use chrono::{Datelike, NaiveDate};
use rusqlite::{params, params_from_iter, types::Value, Connection, Row};

use super::models::*;
use crate::errors::{AppError, AppResult};

pub fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}

pub fn validate_date(value: &str) -> AppResult<NaiveDate> {
    Ok(NaiveDate::parse_from_str(value, "%Y-%m-%d")?)
}

pub fn validate_month(value: &str) -> AppResult<()> {
    NaiveDate::parse_from_str(&format!("{value}-01"), "%Y-%m-%d")?;
    Ok(())
}

pub fn validate_transaction(input: &TransactionInput) -> AppResult<()> {
    if input.amount_cents < 0 {
        return Err(AppError::Validation(
            "amountCents must be non-negative".into(),
        ));
    }
    if !matches!(input.transaction_type.as_str(), "expense" | "income") {
        return Err(AppError::Validation(
            "transactionType must be expense or income".into(),
        ));
    }
    validate_date(&input.date)?;
    Ok(())
}

pub fn transaction_from_row(row: &Row<'_>) -> rusqlite::Result<Transaction> {
    Ok(Transaction {
        id: row.get(0)?,
        account_id: row.get(1)?,
        category_id: row.get(2)?,
        transaction_type: row.get(3)?,
        amount_cents: row.get(4)?,
        date: row.get(5)?,
        description: row.get(6)?,
        notes: row.get(7)?,
        recurring_rule_id: row.get(8)?,
        created_at: row.get(9)?,
        updated_at: row.get(10)?,
    })
}

pub fn get_transaction(connection: &Connection, id: &str) -> AppResult<Transaction> {
    connection.query_row(
        "SELECT id,account_id,category_id,transaction_type,amount_cents,date,description,notes,recurring_rule_id,created_at,updated_at FROM transactions WHERE id=?1",
        [id], transaction_from_row,
    ).map_err(|error| match error { rusqlite::Error::QueryReturnedNoRows => AppError::NotFound("transaction".into()), other => other.into() })
}

pub fn list_transactions(
    connection: &Connection,
    filter: &TransactionFilter,
) -> AppResult<Vec<Transaction>> {
    let mut sql = "SELECT id,account_id,category_id,transaction_type,amount_cents,date,description,notes,recurring_rule_id,created_at,updated_at FROM transactions WHERE 1=1".to_string();
    let mut values: Vec<Value> = Vec::new();
    for (column, value, operator) in [
        ("date", filter.start_date.as_ref(), ">="),
        ("date", filter.end_date.as_ref(), "<="),
        ("category_id", filter.category_id.as_ref(), "="),
        ("account_id", filter.account_id.as_ref(), "="),
        ("transaction_type", filter.transaction_type.as_ref(), "="),
    ] {
        if let Some(value) = value {
            sql.push_str(&format!(" AND {column} {operator} ?"));
            values.push(value.clone().into());
        }
    }
    if let Some(search) = &filter.search {
        sql.push_str(" AND (description LIKE ? OR notes LIKE ?)");
        let pattern = format!("%{search}%");
        values.push(pattern.clone().into());
        values.push(pattern.into());
    }
    sql.push_str(" ORDER BY date DESC, created_at DESC LIMIT ? OFFSET ?");
    values.push(i64::from(filter.limit.unwrap_or(500).min(5000)).into());
    values.push(i64::from(filter.offset.unwrap_or(0)).into());
    let mut statement = connection.prepare(&sql)?;
    let rows = statement.query_map(params_from_iter(values), transaction_from_row)?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn list_accounts(connection: &Connection) -> AppResult<Vec<Account>> {
    let mut statement =
        connection.prepare("SELECT id,name,account_type,currency FROM accounts ORDER BY name")?;
    let accounts = statement
        .query_map([], |r| {
            Ok(Account {
                id: r.get(0)?,
                name: r.get(1)?,
                account_type: r.get(2)?,
                currency: r.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(accounts)
}

pub fn list_categories(connection: &Connection) -> AppResult<Vec<Category>> {
    let mut statement = connection
        .prepare("SELECT id,name,color,icon,category_type FROM categories ORDER BY name")?;
    let categories = statement
        .query_map([], |r| {
            Ok(Category {
                id: r.get(0)?,
                name: r.get(1)?,
                color: r.get(2)?,
                icon: r.get(3)?,
                category_type: r.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(categories)
}

pub fn settings(connection: &Connection) -> AppResult<HashMap<String, String>> {
    let mut statement = connection.prepare("SELECT key,value FROM settings")?;
    let values = statement
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<HashMap<_, _>, _>>()?;
    Ok(values)
}

pub fn list_budgets(connection: &Connection, month: &str) -> AppResult<Vec<Budget>> {
    validate_month(month)?;
    let mut statement = connection.prepare(
        "SELECT b.id,b.category_id,b.month,b.amount_cents,
         COALESCE(SUM(CASE WHEN t.transaction_type='expense' THEN t.amount_cents ELSE 0 END),0),
         b.created_at,b.updated_at FROM budgets b LEFT JOIN transactions t ON t.category_id=b.category_id AND substr(t.date,1,7)=b.month
         WHERE b.month=?1 GROUP BY b.id ORDER BY b.created_at")?;
    let rows = statement.query_map([month], |r| {
        let amount: i64 = r.get(3)?;
        let spent: i64 = r.get(4)?;
        Ok(Budget {
            id: r.get(0)?,
            category_id: r.get(1)?,
            month: r.get(2)?,
            amount_cents: amount,
            spent_cents: spent,
            remaining_cents: amount - spent,
            created_at: r.get(5)?,
            updated_at: r.get(6)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn budget_progress(connection: &Connection, month: &str) -> AppResult<Vec<BudgetProgress>> {
    let budgets = list_budgets(connection, month)?;
    let categories: HashMap<_, _> = list_categories(connection)?
        .into_iter()
        .map(|c| (c.id.clone(), c))
        .collect();
    Ok(budgets
        .into_iter()
        .filter_map(|b| {
            categories.get(&b.category_id).map(|c| BudgetProgress {
                category_id: b.category_id,
                category_name: c.name.clone(),
                color: c.color.clone(),
                budget_cents: b.amount_cents,
                spent_cents: b.spent_cents,
                remaining_cents: b.remaining_cents,
                percent_used: if b.amount_cents == 0 {
                    if b.spent_cents > 0 {
                        100.0
                    } else {
                        0.0
                    }
                } else {
                    b.spent_cents as f64 / b.amount_cents as f64 * 100.0
                },
            })
        })
        .collect())
}

pub fn dashboard(connection: &Connection, month: &str) -> AppResult<DashboardSummary> {
    validate_month(month)?;
    let (income, expense, count) = connection.query_row(
        "SELECT COALESCE(SUM(CASE WHEN transaction_type='income' THEN amount_cents ELSE 0 END),0),COALESCE(SUM(CASE WHEN transaction_type='expense' THEN amount_cents ELSE 0 END),0),COUNT(*) FROM transactions WHERE substr(date,1,7)=?1",
        [month], |r| Ok((r.get::<_,i64>(0)?,r.get::<_,i64>(1)?,r.get::<_,i64>(2)?)))?;
    let mut statement = connection.prepare("SELECT t.category_id,COALESCE(c.name,'Uncategorized'),COALESCE(c.color,'#6c757d'),SUM(t.amount_cents) FROM transactions t LEFT JOIN categories c ON c.id=t.category_id WHERE substr(t.date,1,7)=?1 AND t.transaction_type='expense' GROUP BY t.category_id ORDER BY SUM(t.amount_cents) DESC")?;
    let by_category = statement
        .query_map([month], |r| {
            Ok(CategoryTotal {
                category_id: r.get(0)?,
                category_name: r.get(1)?,
                color: r.get(2)?,
                amount_cents: r.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(DashboardSummary {
        month: month.into(),
        income_cents: income,
        expense_cents: expense,
        net_cents: income - expense,
        transaction_count: count,
        by_category,
        budgets: budget_progress(connection, month)?,
        recent_transactions: list_transactions(
            connection,
            &TransactionFilter {
                start_date: Some(format!("{month}-01")),
                end_date: month_end(month).ok(),
                limit: Some(10),
                ..Default::default()
            },
        )?,
    })
}

pub fn month_end(month: &str) -> AppResult<String> {
    let start = NaiveDate::parse_from_str(&format!("{month}-01"), "%Y-%m-%d")?;
    let (year, next_month) = if start.month() == 12 {
        (start.year() + 1, 1)
    } else {
        (start.year(), start.month() + 1)
    };
    Ok(
        (NaiveDate::from_ymd_opt(year, next_month, 1).unwrap() - chrono::Duration::days(1))
            .format("%Y-%m-%d")
            .to_string(),
    )
}

pub fn analytics(
    connection: &Connection,
    start: &str,
    end: &str,
    group_by: &str,
) -> AppResult<AnalyticsSummary> {
    validate_date(start)?;
    validate_date(end)?;
    let (income,expense)=connection.query_row("SELECT COALESCE(SUM(CASE WHEN transaction_type='income' THEN amount_cents ELSE 0 END),0),COALESCE(SUM(CASE WHEN transaction_type='expense' THEN amount_cents ELSE 0 END),0) FROM transactions WHERE date BETWEEN ?1 AND ?2",params![start,end],|r|Ok((r.get::<_,i64>(0)?,r.get::<_,i64>(1)?)))?;
    let mut category_stmt=connection.prepare("SELECT t.category_id,COALESCE(c.name,'Uncategorized'),COALESCE(c.color,'#6c757d'),SUM(t.amount_cents) FROM transactions t LEFT JOIN categories c ON c.id=t.category_id WHERE t.date BETWEEN ?1 AND ?2 AND t.transaction_type='expense' GROUP BY t.category_id ORDER BY SUM(t.amount_cents) DESC")?;
    let by_category = category_stmt
        .query_map(params![start, end], |r| {
            Ok(CategoryTotal {
                category_id: r.get(0)?,
                category_name: r.get(1)?,
                color: r.get(2)?,
                amount_cents: r.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    let format = match group_by {
        "day" => "%Y-%m-%d",
        "year" => "%Y",
        _ => "%Y-%m",
    };
    let mut period_stmt=connection.prepare(&format!("SELECT strftime('{format}',date),COALESCE(SUM(CASE WHEN transaction_type='income' THEN amount_cents ELSE 0 END),0),COALESCE(SUM(CASE WHEN transaction_type='expense' THEN amount_cents ELSE 0 END),0) FROM transactions WHERE date BETWEEN ?1 AND ?2 GROUP BY 1 ORDER BY 1"))?;
    let periods = period_stmt
        .query_map(params![start, end], |r| {
            let i: i64 = r.get(1)?;
            let e: i64 = r.get(2)?;
            Ok(PeriodTotal {
                period: r.get(0)?,
                income_cents: i,
                expense_cents: e,
                net_cents: i - e,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(AnalyticsSummary {
        start_date: start.into(),
        end_date: end.into(),
        income_cents: income,
        expense_cents: expense,
        net_cents: income - expense,
        by_category,
        periods,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    #[test]
    fn dashboard_and_budget_use_expenses_only() {
        let db = Database::memory().unwrap();
        db.with_connection(|c| {
            let now=now();
            c.execute("INSERT INTO budgets(id,category_id,month,amount_cents,created_at,updated_at) VALUES('b','category-food','2026-07',10000,?1,?1)",[&now])?;
            c.execute("INSERT INTO transactions(id,account_id,category_id,transaction_type,amount_cents,date,description,notes,created_at,updated_at) VALUES('t','account-default','category-food','expense',2500,'2026-07-01','','',?1,?1)",[&now])?;
            c.execute("INSERT INTO transactions(id,account_id,category_id,transaction_type,amount_cents,date,description,notes,created_at,updated_at) VALUES('i','account-default','category-income','income',5000,'2026-07-02','','',?1,?1)",[&now])?;
            let result=dashboard(c,"2026-07")?;
            assert_eq!((result.income_cents,result.expense_cents,result.net_cents),(5000,2500,2500));
            assert_eq!(result.budgets[0].remaining_cents,7500);
            Ok(())
        }).unwrap();
    }
}
