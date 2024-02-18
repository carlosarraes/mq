use crate::routes::{dev, projects};
use axum::{routing::get, Router};
use std::{error::Error, sync::Arc};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod dao;
mod db;
mod handlers;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .compact(),
        )
        .init();

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let db_pool = crate::db::config::init_db().await?;

    let dev_dao = crate::dao::dev::DevDao::new(db_pool.clone());
    let dev_service = Arc::new(crate::services::dev::DevService::new(dev_dao));
    let projects_dao = crate::dao::projects::ProjectsDao::new(db_pool.clone());
    let projects_service = Arc::new(crate::services::projects::ProjectsService::new(
        projects_dao,
    ));

    let app = Router::new()
        .route("/", get(handlers::check::health))
        .nest("/dev", dev::get_routes(dev_service))
        .nest("/projects", projects::get_routes(projects_service))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().level(Level::INFO))
                .on_request(DefaultOnRequest::default().level(Level::INFO))
                .on_response(DefaultOnResponse::default().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
