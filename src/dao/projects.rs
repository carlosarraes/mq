use sqlx::SqlitePool;

use crate::models::projects::{Project, ProjectDto};

pub struct ProjectsDao {
    pool: SqlitePool,
}

impl ProjectsDao {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, insert_project: ProjectDto) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO Project (name, emoji) VALUES (?,?)",
            insert_project.name,
            insert_project.emoji
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<Project>, sqlx::Error> {
        let projects = sqlx::query_as!(Project, "SELECT * FROM Project")
            .fetch_all(&self.pool)
            .await?;

        Ok(projects)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Project, sqlx::Error> {
        let project = sqlx::query_as!(Project, "SELECT * FROM Project WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(project)
    }

    pub async fn update(&self, id: i64, update_project: ProjectDto) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE Project SET name = ?, emoji = ? WHERE id = ?",
            update_project.name,
            update_project.emoji,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM Project WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
