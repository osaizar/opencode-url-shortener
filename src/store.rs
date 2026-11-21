use sqlx::SqlitePool;

use crate::error::{ApiError, AppResult};

#[derive(Clone)]
pub struct Store {
    pool: SqlitePool,
}

impl Store {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Returns true when the mapping was inserted, false when the code already exists.
    pub async fn insert(&self, code: &str, target_url: &str) -> AppResult<bool> {
        let result = sqlx::query("INSERT OR IGNORE INTO urls (code, target_url) VALUES (?, ?)")
            .bind(code)
            .bind(target_url)
            .execute(&self.pool)
            .await
            .map_err(|_| ApiError::Internal)?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn find(&self, code: &str) -> AppResult<Option<String>> {
        let row: Option<(String,)> = sqlx::query_as("SELECT target_url FROM urls WHERE code = ?")
            .bind(code)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| ApiError::Internal)?;
        Ok(row.map(|record| record.0))
    }
}
