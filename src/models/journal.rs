use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Journal {
    pub id: i64,
    pub title: String,
    pub comment: String,
    pub day: chrono::NaiveDate,
    pub dev: String,
    pub dev_id: i64,
    pub project: String,
    pub project_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct JournalDto {
    pub title: String,
    pub comment: String,
    pub day: chrono::NaiveDate,
    pub dev_id: i64,
    pub project_id: i64,
}

#[derive(Deserialize)]
pub struct DateQuery {
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}
