use crate::{
    database::{queries, Database},
    errors::{AppError, AppResult},
};
use serde::Deserialize;
use std::collections::HashMap;
use tauri::State;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsInput {
    pub values: HashMap<String, String>,
}

#[tauri::command]
pub fn update_settings(
    database: State<'_, Database>,
    input: SettingsInput,
) -> AppResult<HashMap<String, String>> {
    if input.values.keys().any(|k| k.trim().is_empty()) {
        return Err(AppError::Validation("setting keys cannot be empty".into()));
    }
    database.with_connection(|c|{let tx=c.transaction()?;for(key,value)in input.values{tx.execute("INSERT INTO settings(key,value,updated_at) VALUES(?1,?2,?3) ON CONFLICT(key) DO UPDATE SET value=excluded.value,updated_at=excluded.updated_at",rusqlite::params![key,value,queries::now()])?;}tx.commit()?;queries::settings(c)})
}
