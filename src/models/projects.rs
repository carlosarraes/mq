use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: i64,
    pub emoji: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectDto {
    pub emoji: String,
    pub name: String,
}
