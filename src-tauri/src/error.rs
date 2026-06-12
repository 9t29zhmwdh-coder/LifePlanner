use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum LpError {
    #[error("Database error: {0}")]
    Db(String),
    #[error("Calendar error: {0}")]
    Calendar(String),
    #[error("AI error: {0}")]
    Ai(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("{0}")]
    Other(String),
}

impl From<lp_core::db::DbError> for LpError {
    fn from(e: lp_core::db::DbError) -> Self { Self::Db(e.to_string()) }
}

pub type LpResult<T> = Result<T, LpError>;
