use serde::{Serialize, Serializer};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("tauri error: {0}")]
    Tauri(#[from] tauri::Error),
    #[error("api limit reached: {0}")]
    ApiLimit(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn code(&self) -> u32 {
        match self {
            Self::Database(_) => 1000,
            Self::Io(_) => 1001,
            Self::Network(_) => 1002,
            Self::Tauri(_) => 1003,
            Self::ApiLimit(_) => 2000,
            Self::InvalidInput(_) => 3000,
            Self::NotFound(_) => 4000,
            Self::Internal(_) => 5000,
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let error_json = json!({
            "code": self.code(),
            "message": self.to_string()
        });
        error_json.serialize(serializer)
    }
}

pub type AppResult<T> = Result<T, AppError>;
