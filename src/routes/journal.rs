use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};

use crate::{handlers, services::journal::JournalService};

pub fn get_routes(service: Arc<JournalService>) -> Router {
    Router::new()
        .route(
            "/",
            post(handlers::journal::create).get(handlers::journal::get_all),
        )
        .route("/date", get(handlers::journal::get_all_by_date_range))
        .route(
            "/dev/:dev_id",
            get(handlers::journal::get_all_by_dev_id_and_date_range),
        )
        .route(
            "/project/:project_id",
            get(handlers::journal::get_by_project_id),
        )
        .route("/toml/:dev_id", get(handlers::journal::serialize_to_toml))
        .route("/yaml", get(handlers::journal::serialize_to_yaml))
        .route("/md", get(handlers::journal::serialize_to_md))
        .route(
            "/:id",
            get(handlers::journal::get_by_id)
                .put(handlers::journal::update)
                .delete(handlers::journal::delete),
        )
        .layer(Extension(service))
}
