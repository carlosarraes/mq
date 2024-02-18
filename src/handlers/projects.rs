use std::sync::Arc;

use axum::{extract, http::StatusCode, Json};

use crate::{
    models::projects::{Project, ProjectDto},
    services::projects::ProjectsService,
};

pub async fn create(
    extract::Extension(service): extract::Extension<Arc<ProjectsService>>,
    Json(project): Json<ProjectDto>,
) -> Result<StatusCode, StatusCode> {
    match service.create(project).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all(
    extract::Extension(service): extract::Extension<Arc<ProjectsService>>,
) -> Result<Json<Vec<Project>>, StatusCode> {
    match service.get_all().await {
        Ok(projects) => Ok(Json(projects)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_by_id(
    extract::Extension(service): extract::Extension<Arc<ProjectsService>>,
    extract::Path(id): extract::Path<i64>,
) -> Result<Json<Project>, StatusCode> {
    match service.get_by_id(id).await {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update(
    extract::Extension(service): extract::Extension<Arc<ProjectsService>>,
    extract::Path(id): extract::Path<i64>,
    Json(project): Json<ProjectDto>,
) -> Result<StatusCode, StatusCode> {
    match service.update(id, project).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete(
    extract::Extension(service): extract::Extension<Arc<ProjectsService>>,
    extract::Path(id): extract::Path<i64>,
) -> Result<StatusCode, StatusCode> {
    match service.delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
