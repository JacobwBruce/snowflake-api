mod handlers;
mod middleware;
mod models;
mod repositories;

use axum::http;
use axum::middleware::from_fn;
use axum::routing::{get, Router};
use sqlx::mysql::MySqlPoolOptions;

use handlers::capacity::capacity_router;
use middleware::authorization::auth;
use middleware::logger::create_logger;
use repositories::capacity_repo::CapacityRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // only checking for the presence of the env var, do not need to store in a variable
    std::env::var("ACCESS_KEY").expect("missing ACCESS_KEY env");

    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let capacity_repo = CapacityRepository { db: pool };

    let app = Router::new()
        .route("/", get(health_check))
        .nest("/capacity", capacity_router(&capacity_repo))
        .layer(create_logger())
        .layer(from_fn(auth));

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
