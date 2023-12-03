mod handlers;
mod models;

use axum::http;
use axum::routing::{get, Router};

use handlers::capacity::capacity_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 3000;
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/", get(health_check))
        .nest("/capacity", capacity_router());

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health_check() -> http::StatusCode {
    http::StatusCode::OK
}
