use std::sync::Arc;

use axum::{routing::post, Extension, Router};

use crate::{handlers, services::projects::ProjectsService};

pub fn get_routes(service: Arc<ProjectsService>) -> Router {
    Router::new()
        .route(
            "/",
            post(handlers::projects::create).get(handlers::projects::get_all),
        )
        .route(
            "/:id",
            post(handlers::projects::update)
                .get(handlers::projects::get_by_id)
                .delete(handlers::projects::delete),
        )
        .layer(Extension(service))
}
