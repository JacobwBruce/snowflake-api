mod handlers;
mod models;

use axum::http;
use axum::routing::{get, Router};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use handlers::capacity::capacity_router;

pub struct AppState {
    pool: MySqlPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app_state = AppState { pool };

    let app = Router::new()
        .route("/", get(health_check))
        .nest("/capacity", capacity_router(&app_state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR)),
        );

    let server = axum::Server::bind(&addr.parse().unwrap()).serve(app.into_make_service());

    tracing::info!("Server running on port {}", port);

    if let Err(err) = server.await {
        eprintln!("Server failed to start: {}", err);
        std::process::exit(1);
    }

    Ok(())
}

async fn health_check() -> http::StatusCode {
    http::StatusCode::OK
}
