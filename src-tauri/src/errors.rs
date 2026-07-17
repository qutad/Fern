use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("invalid input: {0}")]
    Validation(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("date error: {0}")]
    Date(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(value: chrono::ParseError) -> Self {
        Self::Date(value.to_string())
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::Validation(value.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

pub fn poisoned_lock(error: impl fmt::Display) -> AppError {
    AppError::Database(rusqlite::Error::InvalidParameterName(format!(
        "database lock poisoned: {error}"
    )))
}
