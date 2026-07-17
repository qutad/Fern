use crate::{
    database::{migrations, Database},
    errors::{AppError, AppResult},
};
use rusqlite::{backup::Backup, Connection};
use serde::{Deserialize, Serialize};
use std::{path::Path, time::Duration};
use tauri::State;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupPathInput {
    pub path: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupResult {
    pub path: String,
}

fn validate_path(path: &str) -> AppResult<()> {
    if path.trim().is_empty() {
        Err(AppError::Validation("path cannot be empty".into()))
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn create_backup(
    database: State<'_, Database>,
    input: BackupPathInput,
) -> AppResult<BackupResult> {
    validate_path(&input.path)?;
    if Path::new(&input.path) == database.path {
        return Err(AppError::Validation(
            "backup path must differ from database path".into(),
        ));
    }
    if let Some(parent) = Path::new(&input.path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    database.with_connection(|source| {
        let mut destination = Connection::open(&input.path)?;
        let backup = Backup::new(source, &mut destination)?;
        backup.run_to_completion(100, Duration::from_millis(10), None)?;
        Ok(())
    })?;
    Ok(BackupResult { path: input.path })
}

#[tauri::command]
pub fn restore_backup(
    database: State<'_, Database>,
    input: BackupPathInput,
) -> AppResult<BackupResult> {
    validate_path(&input.path)?;
    if !Path::new(&input.path).is_file() {
        return Err(AppError::NotFound("backup file".into()));
    }
    let source = Connection::open(&input.path)?;
    source.query_row("PRAGMA integrity_check", [], |r| {
        let result: String = r.get(0)?;
        if result == "ok" {
            Ok(())
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    })?;
    database.with_connection(|destination| {
        {
            let backup = Backup::new(&source, destination)?;
            backup.run_to_completion(100, Duration::from_millis(10), None)?;
        }
        destination.pragma_update(None, "foreign_keys", "ON")?;
        migrations::migrate(destination)?;
        Ok(())
    })?;
    Ok(BackupResult { path: input.path })
}
