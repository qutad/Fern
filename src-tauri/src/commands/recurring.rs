use chrono::NaiveDate;
use rusqlite::{params, Connection, Row};
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

use crate::{
    database::{
        models::{GenerateResult, RecurringRule, RecurringRuleInput},
        queries, Database,
    },
    errors::{AppError, AppResult},
    services::recurrence_dates,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringIdInput {
    pub id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecurringInput {
    pub id: String,
    #[serde(flatten)]
    pub rule: RecurringRuleInput,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateDueInput {
    pub through_date: Option<String>,
}

fn from_row(r: &Row<'_>) -> rusqlite::Result<RecurringRule> {
    Ok(RecurringRule {
        id: r.get(0)?,
        account_id: r.get(1)?,
        category_id: r.get(2)?,
        transaction_type: r.get(3)?,
        amount_cents: r.get(4)?,
        description: r.get(5)?,
        notes: r.get(6)?,
        frequency: r.get(7)?,
        interval_count: r.get(8)?,
        start_date: r.get(9)?,
        end_date: r.get(10)?,
        active: r.get::<_, i64>(11)? != 0,
        created_at: r.get(12)?,
        updated_at: r.get(13)?,
    })
}

fn validate(input: &RecurringRuleInput) -> AppResult<()> {
    queries::validate_transaction(&crate::database::models::TransactionInput {
        account_id: input.account_id.clone(),
        category_id: input.category_id.clone(),
        transaction_type: input.transaction_type.clone(),
        amount_cents: input.amount_cents,
        date: input.start_date.clone(),
        description: input.description.clone(),
        notes: input.notes.clone(),
    })?;
    if input.interval_count <= 0 {
        return Err(AppError::Validation(
            "intervalCount must be positive".into(),
        ));
    }
    if !matches!(
        input.frequency.as_str(),
        "daily" | "weekly" | "monthly" | "yearly"
    ) {
        return Err(AppError::Validation("invalid frequency".into()));
    }
    if let Some(end) = &input.end_date {
        if queries::validate_date(end)? < queries::validate_date(&input.start_date)? {
            return Err(AppError::Validation(
                "endDate must not precede startDate".into(),
            ));
        }
    }
    Ok(())
}

fn get(connection: &Connection, id: &str) -> AppResult<RecurringRule> {
    connection.query_row("SELECT id,account_id,category_id,transaction_type,amount_cents,description,notes,frequency,interval_count,start_date,end_date,active,created_at,updated_at FROM recurring_rules WHERE id=?1",[id],from_row).map_err(|e|match e{rusqlite::Error::QueryReturnedNoRows=>AppError::NotFound("recurring rule".into()),e=>e.into()})
}

#[tauri::command]
pub fn list_recurring_rules(database: State<'_, Database>) -> AppResult<Vec<RecurringRule>> {
    database.with_connection(|c| {
        let mut statement=c.prepare("SELECT id,account_id,category_id,transaction_type,amount_cents,description,notes,frequency,interval_count,start_date,end_date,active,created_at,updated_at FROM recurring_rules ORDER BY active DESC,start_date")?;
        let rules = statement.query_map([], from_row)?.collect::<Result<Vec<_>, _>>()?;
        Ok(rules)
    })
}

#[tauri::command]
pub fn create_recurring_rule(
    database: State<'_, Database>,
    input: RecurringRuleInput,
) -> AppResult<RecurringRule> {
    validate(&input)?;
    database.with_connection(|c|{let id=Uuid::new_v4().to_string();let now=queries::now();c.execute("INSERT INTO recurring_rules(id,account_id,category_id,transaction_type,amount_cents,description,notes,frequency,interval_count,start_date,end_date,active,created_at,updated_at) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?13)",params![id,input.account_id,input.category_id,input.transaction_type,input.amount_cents,input.description,input.notes,input.frequency,input.interval_count,input.start_date,input.end_date,input.active,now])?;get(c,&id)})
}

#[tauri::command]
pub fn update_recurring_rule(
    database: State<'_, Database>,
    input: UpdateRecurringInput,
) -> AppResult<RecurringRule> {
    validate(&input.rule)?;
    database.with_connection(|c|{let r=input.rule;let changed=c.execute("UPDATE recurring_rules SET account_id=?2,category_id=?3,transaction_type=?4,amount_cents=?5,description=?6,notes=?7,frequency=?8,interval_count=?9,start_date=?10,end_date=?11,active=?12,updated_at=?13 WHERE id=?1",params![input.id,r.account_id,r.category_id,r.transaction_type,r.amount_cents,r.description,r.notes,r.frequency,r.interval_count,r.start_date,r.end_date,r.active,queries::now()])?;if changed==0{return Err(AppError::NotFound("recurring rule".into()));}get(c,&input.id)})
}

#[tauri::command]
pub fn delete_recurring_rule(
    database: State<'_, Database>,
    input: RecurringIdInput,
) -> AppResult<()> {
    database.with_connection(|c| {
        if c.execute("DELETE FROM recurring_rules WHERE id=?1", [input.id])? == 0 {
            Err(AppError::NotFound("recurring rule".into()))
        } else {
            Ok(())
        }
    })
}

pub fn generate(connection: &mut Connection, through: &str) -> AppResult<GenerateResult> {
    let through_date = queries::validate_date(through)?;
    let rules = {
        let mut statement=connection.prepare("SELECT id,account_id,category_id,transaction_type,amount_cents,description,notes,frequency,interval_count,start_date,end_date,active,created_at,updated_at FROM recurring_rules WHERE active=1")?;
        let rules = statement
            .query_map([], from_row)?
            .collect::<Result<Vec<_>, _>>()?;
        rules
    };
    let tx = connection.transaction()?;
    let mut generated = 0;
    for rule in rules {
        let start = NaiveDate::parse_from_str(&rule.start_date, "%Y-%m-%d")?;
        let end = rule
            .end_date
            .as_deref()
            .map(|v| NaiveDate::parse_from_str(v, "%Y-%m-%d"))
            .transpose()?;
        for date in recurrence_dates(
            start,
            end,
            through_date,
            &rule.frequency,
            rule.interval_count,
        )? {
            let date = date.to_string();
            let occurrence_id = Uuid::new_v5(
                &Uuid::NAMESPACE_OID,
                format!("{}:{date}", rule.id).as_bytes(),
            )
            .to_string();
            let transaction_id = Uuid::new_v5(
                &Uuid::NAMESPACE_URL,
                format!("{}:{date}", rule.id).as_bytes(),
            )
            .to_string();
            let now = queries::now();
            let inserted=tx.execute("INSERT OR IGNORE INTO transactions(id,account_id,category_id,transaction_type,amount_cents,date,description,notes,recurring_rule_id,created_at,updated_at) SELECT ?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?10 WHERE NOT EXISTS(SELECT 1 FROM recurring_occurrences WHERE recurring_rule_id=?9 AND occurrence_date=?6)",params![transaction_id,rule.account_id,rule.category_id,rule.transaction_type,rule.amount_cents,date,rule.description,rule.notes,rule.id,now])?;
            if inserted > 0 {
                tx.execute("INSERT INTO recurring_occurrences(id,recurring_rule_id,occurrence_date,transaction_id,created_at) VALUES(?1,?2,?3,?4,?5)",params![occurrence_id,rule.id,date,transaction_id,now])?;
                generated += 1;
            }
        }
    }
    tx.commit()?;
    Ok(GenerateResult {
        generated,
        through_date: through.into(),
    })
}

#[tauri::command]
pub fn generate_recurring_due(
    database: State<'_, Database>,
    input: Option<GenerateDueInput>,
) -> AppResult<GenerateResult> {
    let through = input
        .and_then(|v| v.through_date)
        .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());
    database.with_connection(|c| generate(c, &through))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;
    #[test]
    fn generation_is_idempotent() {
        let db = Database::memory().unwrap();
        db.with_connection(|c|{let now=queries::now();c.execute("INSERT INTO recurring_rules(id,account_id,transaction_type,amount_cents,description,notes,frequency,interval_count,start_date,active,created_at,updated_at) VALUES('r','account-default','expense',100,'x','','monthly',1,'2026-01-31',1,?1,?1)",[now])?;assert_eq!(generate(c,"2026-03-31")?.generated,3);assert_eq!(generate(c,"2026-03-31")?.generated,0);Ok(())}).unwrap();
    }
}
