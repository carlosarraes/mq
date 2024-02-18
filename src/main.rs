use axum::{routing::get, Router};

mod db;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let _db_pool = crate::db::config::init_db().await?;

    let app = Router::new().route("/", get(handlers::check::health));

    println!("Server running on port {port}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
