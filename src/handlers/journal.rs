use std::sync::Arc;

use axum::{extract, http::StatusCode, Json};

use crate::{
    models::journal::{Journal, JournalDto},
    services::journal::JournalService,
};

pub async fn create(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    Json(journal): Json<JournalDto>,
) -> Result<StatusCode, StatusCode> {
    match service.create(journal).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
) -> Result<Json<Vec<Journal>>, StatusCode> {
    match service.get_all().await {
        Ok(journals) => Ok(Json(journals)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_by_dev_id(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Path(dev_id): extract::Path<i64>,
) -> Result<Json<Vec<Journal>>, StatusCode> {
    match service.get_all_by_dev_id(dev_id).await {
        Ok(journal) => Ok(Json(journal)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_by_project_id(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Path(project_id): extract::Path<i64>,
) -> Result<Json<Vec<Journal>>, StatusCode> {
    match service.get_all_by_project_id(project_id).await {
        Ok(journal) => Ok(Json(journal)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_by_id(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Path(id): extract::Path<i64>,
) -> Result<Json<Journal>, StatusCode> {
    match service.get_by_id(id).await {
        Ok(journal) => Ok(Json(journal)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Path(id): extract::Path<i64>,
    Json(journal): Json<JournalDto>,
) -> Result<StatusCode, StatusCode> {
    match service.update(id, journal).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Path(id): extract::Path<i64>,
) -> Result<StatusCode, StatusCode> {
    match service.delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn serialize_to_toml(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Path(dev_id): extract::Path<i64>,
) -> Result<String, StatusCode> {
    tracing::info!("dev_id: {}", dev_id);
    match service.serialize_to_toml(dev_id).await {
        Ok(journal) => Ok(journal),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn serialize_to_yaml(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Path(dev_id): extract::Path<i64>,
) -> Result<String, StatusCode> {
    match service.serialize_to_yaml(dev_id).await {
        Ok(journal) => Ok(journal),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
