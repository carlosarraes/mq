use std::sync::Arc;

use axum::{extract, http::StatusCode, Json};

use crate::{
    models::dev::{Dev, DevDto},
    services::dev::DevService,
};

pub async fn create(
    extract::Extension(service): extract::Extension<Arc<DevService>>,
    Json(dev_name): Json<DevDto>,
) -> Result<StatusCode, StatusCode> {
    match service.create(dev_name).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all(
    extract::Extension(service): extract::Extension<Arc<DevService>>,
) -> Json<Vec<Dev>> {
    let devs = service.get_all().await.unwrap();
    Json(devs)
}

pub async fn get_by_id(
    extract::Extension(service): extract::Extension<Arc<DevService>>,
    extract::Path(id): extract::Path<i64>,
) -> Result<Json<Dev>, StatusCode> {
    match service.get_by_id(id).await {
        Ok(dev) => Ok(Json(dev)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update(
    extract::Extension(service): extract::Extension<Arc<DevService>>,
    extract::Path(id): extract::Path<i64>,
    Json(dev_name): Json<DevDto>,
) -> Result<StatusCode, StatusCode> {
    match service.update(id, dev_name).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete(
    extract::Extension(service): extract::Extension<Arc<DevService>>,
    extract::Path(id): extract::Path<i64>,
) -> Result<StatusCode, StatusCode> {
    match service.delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
