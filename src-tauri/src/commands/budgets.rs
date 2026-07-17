use rusqlite::params;
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

use crate::{
    database::{
        models::{Budget, BudgetInput},
        queries, Database,
    },
    errors::{AppError, AppResult},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetMonthInput {
    pub month: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetIdInput {
    pub id: String,
}

#[tauri::command]
pub fn list_budgets(
    database: State<'_, Database>,
    input: BudgetMonthInput,
) -> AppResult<Vec<Budget>> {
    database.with_connection(|c| queries::list_budgets(c, &input.month))
}

#[tauri::command]
pub fn upsert_budget(database: State<'_, Database>, input: BudgetInput) -> AppResult<Budget> {
    queries::validate_month(&input.month)?;
    if input.amount_cents < 0 {
        return Err(AppError::Validation(
            "amountCents must be non-negative".into(),
        ));
    }
    database.with_connection(|c|{let id=Uuid::new_v4().to_string();let now=queries::now();c.execute("INSERT INTO budgets(id,category_id,month,amount_cents,created_at,updated_at) VALUES(?1,?2,?3,?4,?5,?5) ON CONFLICT(category_id,month) DO UPDATE SET amount_cents=excluded.amount_cents,updated_at=excluded.updated_at",params![id,input.category_id,input.month,input.amount_cents,now])?;queries::list_budgets(c,&input.month)?.into_iter().find(|b|b.category_id==input.category_id).ok_or_else(||AppError::NotFound("budget".into()))})
}

#[tauri::command]
pub fn delete_budget(database: State<'_, Database>, input: BudgetIdInput) -> AppResult<()> {
    database.with_connection(|c| {
        if c.execute("DELETE FROM budgets WHERE id=?1", [input.id])? == 0 {
            Err(AppError::NotFound("budget".into()))
        } else {
            Ok(())
        }
    })
}
