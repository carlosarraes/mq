use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};

use crate::{handlers, services::dev::DevService};

pub fn get_routes(service: Arc<DevService>) -> Router {
    Router::new()
        .route("/", post(handlers::dev::create).get(handlers::dev::get_all))
        .route(
            "/:id",
            get(handlers::dev::get_by_id)
                .patch(handlers::dev::update)
                .delete(handlers::dev::delete),
        )
        .layer(Extension(service))
}
