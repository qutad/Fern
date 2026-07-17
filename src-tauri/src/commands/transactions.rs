use rusqlite::params;
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

use crate::{
    database::{
        models::{Transaction, TransactionFilter, TransactionInput},
        queries, Database,
    },
    errors::{AppError, AppResult},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionIdInput {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransactionInput {
    pub id: String,
    #[serde(flatten)]
    pub transaction: TransactionInput,
}

#[tauri::command]
pub fn list_transactions(
    database: State<'_, Database>,
    filter: Option<TransactionFilter>,
) -> AppResult<Vec<Transaction>> {
    database.with_connection(|c| queries::list_transactions(c, &filter.unwrap_or_default()))
}

#[tauri::command]
pub fn create_transaction(
    database: State<'_, Database>,
    input: TransactionInput,
) -> AppResult<Transaction> {
    queries::validate_transaction(&input)?;
    database.with_connection(|c|{
        let id=Uuid::new_v4().to_string();let now=queries::now();
        c.execute("INSERT INTO transactions(id,account_id,category_id,transaction_type,amount_cents,date,description,notes,created_at,updated_at) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?9)",params![id,input.account_id,input.category_id,input.transaction_type,input.amount_cents,input.date,input.description,input.notes,now])?;
        queries::get_transaction(c,&id)
    })
}

#[tauri::command]
pub fn update_transaction(
    database: State<'_, Database>,
    input: UpdateTransactionInput,
) -> AppResult<Transaction> {
    queries::validate_transaction(&input.transaction)?;
    database.with_connection(|c|{
        let t=input.transaction;let changed=c.execute("UPDATE transactions SET account_id=?2,category_id=?3,transaction_type=?4,amount_cents=?5,date=?6,description=?7,notes=?8,updated_at=?9 WHERE id=?1",params![input.id,t.account_id,t.category_id,t.transaction_type,t.amount_cents,t.date,t.description,t.notes,queries::now()])?;
        if changed==0{return Err(AppError::NotFound("transaction".into()));} queries::get_transaction(c,&input.id)
    })
}

#[tauri::command]
pub fn delete_transaction(
    database: State<'_, Database>,
    input: TransactionIdInput,
) -> AppResult<()> {
    database.with_connection(|c| {
        if c.execute("DELETE FROM transactions WHERE id=?1", [input.id])? == 0 {
            Err(AppError::NotFound("transaction".into()))
        } else {
            Ok(())
        }
    })
}
