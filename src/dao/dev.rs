use sqlx::SqlitePool;

use crate::models::dev::{Dev, DevDto};

pub struct DevDao {
    pool: SqlitePool,
}

impl DevDao {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, insert_dev: DevDto) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO Dev (name) VALUES (?)", insert_dev.name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<Dev>, sqlx::Error> {
        let devs = sqlx::query_as!(Dev, "SELECT id,name FROM Dev")
            .fetch_all(&self.pool)
            .await?;

        Ok(devs)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Dev, sqlx::Error> {
        let dev = sqlx::query_as!(Dev, "SELECT id,name FROM Dev WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(dev)
    }

    pub async fn update(&self, id: i64, update_dev: DevDto) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE Dev SET name = ? WHERE id = ?", update_dev.name, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM Dev WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
