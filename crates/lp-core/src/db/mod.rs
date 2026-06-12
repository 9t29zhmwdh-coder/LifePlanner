use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::path::Path;
use thiserror::Error;

pub mod queries;
pub use queries::*;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Migrate error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

pub type DbResult<T> = Result<T, DbError>;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn open(db_path: &Path) -> DbResult<Self> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let url = format!("sqlite://{}?mode=rwc", db_path.display());
        let pool = SqlitePool::connect(&url).await?;
        sqlx::migrate!("./src/db/migrations").run(&pool).await?;
        Ok(Self { pool })
    }
}
