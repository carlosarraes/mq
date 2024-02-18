use sqlx::SqlitePool;

use crate::models::journal::{Journal, JournalDto};

pub struct JournalDao {
    pool: SqlitePool,
}

impl JournalDao {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, insert_journal: JournalDto) -> Result<(), sqlx::Error> {
        let day = insert_journal.day.format("%Y-%m-%d").to_string();

        sqlx::query!(
            "INSERT INTO Journal (title, comment, day, dev_id, project_id) VALUES (?,?,?,?,?)",
            insert_journal.title,
            insert_journal.comment,
            day,
            insert_journal.dev_id,
            insert_journal.project_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<Journal>, sqlx::Error> {
        let journals = sqlx::query_as!(
            Journal,
            r#"
            SELECT 
                Journal.id, 
                Journal.day, 
                Journal.title, 
                Journal.comment, 
                Dev.name AS "dev", 
                Journal.dev_id AS "dev_id!", 
                Project.name AS "project", 
                Journal.project_id AS "project_id!"
            FROM Journal
            INNER JOIN Dev ON Journal.dev_id = Dev.id
            INNER JOIN Project ON Journal.project_id = Project.id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(journals)
    }

    pub async fn get_all_by_dev_id(&self, dev_id: i64) -> Result<Vec<Journal>, sqlx::Error> {
        let journals = sqlx::query_as!(
            Journal,
            r#"
            SELECT 
                Journal.id, 
                Journal.day, 
                Journal.title, 
                Journal.comment, 
                Dev.name AS "dev", 
                Journal.dev_id AS "dev_id!", 
                Project.name AS "project", 
                Journal.project_id AS "project_id!"
            FROM Journal
            INNER JOIN Dev ON Journal.dev_id = Dev.id
            INNER JOIN Project ON Journal.project_id = Project.id
            WHERE Journal.dev_id = ?
            "#,
            dev_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(journals)
    }

    pub async fn get_all_by_project_id(
        &self,
        project_id: i64,
    ) -> Result<Vec<Journal>, sqlx::Error> {
        let journals = sqlx::query_as!(
            Journal,
            r#"
            SELECT 
                Journal.id, 
                Journal.day, 
                Journal.title, 
                Journal.comment, 
                Dev.name AS "dev", 
                Journal.dev_id AS "dev_id!", 
                Project.name AS "project", 
                Journal.project_id AS "project_id!"
            FROM Journal
            INNER JOIN Dev ON Journal.dev_id = Dev.id
            INNER JOIN Project ON Journal.project_id = Project.id
            WHERE Journal.project_id = ?
            "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(journals)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Journal, sqlx::Error> {
        let journal = sqlx::query_as!(
            Journal,
            r#"
            SELECT 
                Journal.id, 
                Journal.day, 
                Journal.title, 
                Journal.comment, 
                Dev.name AS "dev", 
                Journal.dev_id AS "dev_id!", 
                Project.name AS "project", 
                Journal.project_id AS "project_id!"
            FROM Journal
            INNER JOIN Dev ON Journal.dev_id = Dev.id
            INNER JOIN Project ON Journal.project_id = Project.id
            WHERE Journal.id = ?
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(journal)
    }

    pub async fn update(&self, id: i64, update_journal: JournalDto) -> Result<(), sqlx::Error> {
        let day = update_journal.day.format("%Y-%m-%d").to_string();

        sqlx::query!(
            "UPDATE Journal SET title = ?, comment = ?, day = ?, dev_id = ?, project_id = ? WHERE id = ?",
            update_journal.title,
            update_journal.comment,
            day,
            update_journal.dev_id,
            update_journal.project_id,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM Journal WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
