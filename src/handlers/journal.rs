use std::sync::Arc;

use crate::{
    models::journal::{DateQuery, Journal, JournalDto},
    services::journal::JournalService,
};
use axum::{extract, http::StatusCode, Json};

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

pub async fn get_all_by_date_range(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Query(date_query): extract::Query<DateQuery>,
) -> Result<Json<Vec<Journal>>, StatusCode> {
    let DateQuery {
        start_date,
        end_date,
    } = date_query;

    match service.get_all_by_date_range(start_date, end_date).await {
        Ok(journal) => Ok(Json(journal)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all_by_dev_id_and_date_range(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Path(dev_id): extract::Path<i64>,
    extract::Query(date_query): extract::Query<DateQuery>,
) -> Result<Json<Vec<Journal>>, StatusCode> {
    let DateQuery {
        start_date,
        end_date,
    } = date_query;

    match service
        .get_all_by_dev_id_and_date_range(dev_id, start_date, end_date)
        .await
    {
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
    extract::Query(date_query): extract::Query<DateQuery>,
) -> Result<String, StatusCode> {
    let DateQuery {
        start_date,
        end_date,
    } = date_query;

    match service.serialize_to_toml(start_date, end_date).await {
        Ok(journal) => Ok(journal),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn serialize_to_yaml(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Query(date_query): extract::Query<DateQuery>,
) -> Result<String, StatusCode> {
    let DateQuery {
        start_date,
        end_date,
    } = date_query;

    match service.serialize_to_yaml(start_date, end_date).await {
        Ok(journal) => Ok(journal),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn serialize_to_md(
    extract::Extension(service): extract::Extension<Arc<JournalService>>,
    extract::Query(date_query): extract::Query<DateQuery>,
) -> Result<String, StatusCode> {
    let DateQuery {
        start_date,
        end_date,
    } = date_query;

    match service.serialize_to_md(start_date, end_date).await {
        Ok(journal) => Ok(journal),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
