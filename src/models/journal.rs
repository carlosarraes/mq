use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Journal {
    pub id: i64,
    pub comment: String,
    pub day: chrono::NaiveDate,
    pub dev: String,
    pub dev_id: i64,
    pub project: String,
    pub emoji: String,
    pub project_id: i64,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct JournalDto {
    pub comment: String,
    pub day: chrono::NaiveDate,
    pub dev_id: i64,
    pub project_id: i64,
    pub status: String,
}

#[derive(Deserialize)]
pub struct DateQuery {
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}
