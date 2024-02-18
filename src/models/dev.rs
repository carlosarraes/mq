use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Dev {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct DevDto {
    pub name: String,
}
