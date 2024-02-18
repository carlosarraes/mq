use crate::{
    dao::projects::ProjectsDao,
    models::projects::{Project, ProjectDto},
};

pub struct ProjectsService {
    project_dao: ProjectsDao,
}

impl ProjectsService {
    pub fn new(project_dao: ProjectsDao) -> Self {
        Self { project_dao }
    }

    pub async fn create(&self, insert_project: ProjectDto) -> Result<(), sqlx::Error> {
        self.project_dao.create(insert_project).await
    }

    pub async fn get_all(&self) -> Result<Vec<Project>, sqlx::Error> {
        self.project_dao.get_all().await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Project, sqlx::Error> {
        self.project_dao.get_by_id(id).await
    }

    pub async fn update(&self, id: i64, update_project: ProjectDto) -> Result<(), sqlx::Error> {
        self.project_dao.update(id, update_project).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        self.project_dao.delete(id).await
    }
}
