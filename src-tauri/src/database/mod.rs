pub mod migrations;
pub mod models;
pub mod queries;

use std::{path::PathBuf, sync::Mutex};

use rusqlite::Connection;

use crate::errors::{poisoned_lock, AppResult};

pub struct Database {
    pub path: PathBuf,
    connection: Mutex<Connection>,
}

impl Database {
    pub fn open(path: PathBuf) -> AppResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let connection = Connection::open(&path)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        migrations::migrate(&connection)?;
        Ok(Self {
            path,
            connection: Mutex::new(connection),
        })
    }

    #[cfg(test)]
    pub fn memory() -> AppResult<Self> {
        let connection = Connection::open_in_memory()?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        migrations::migrate(&connection)?;
        Ok(Self {
            path: PathBuf::from(":memory:"),
            connection: Mutex::new(connection),
        })
    }

    pub fn with_connection<T>(
        &self,
        f: impl FnOnce(&mut Connection) -> AppResult<T>,
    ) -> AppResult<T> {
        let mut connection = self.connection.lock().map_err(poisoned_lock)?;
        f(&mut connection)
    }
}
