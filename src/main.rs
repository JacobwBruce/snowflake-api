mod handlers;
mod middleware;
mod models;
mod repositories;

use axum::http;
use axum::middleware::from_fn;
use axum::routing::{get, Router};
use snowflake_connector_rs::SnowflakeAuthMethod;
use snowflake_connector_rs::{SnowflakeClient, SnowflakeClientConfig};

use handlers::capacity::capacity_router;
use middleware::authorization::auth;
use middleware::logger::create_logger;
use repositories::capacity_repo::CapacityRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // only checking for the presence of the env var, do not need to store in a variable
    std::env::var("ACCESS_KEY").expect("missing ACCESS_KEY env");

    let client = create_snowflake_client().await?;
    let session = Box::new(client.create_session().await?);
    let capacity_repo = CapacityRepository {
        session: Box::leak(session),
    };

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

async fn create_snowflake_client() -> Result<SnowflakeClient, Box<dyn std::error::Error>> {
    println!(
        "username: {}",
        std::env::var("SNOWFLAKE_USERNAME").expect("missing SNOWFLAKE_USERNAME env"),
    );
    println!(
        "password: {}",
        std::env::var("SNOWFLAKE_PASSWORD").expect("missing SNOWFLAKE_PASSWORD env"),
    );
    let client = SnowflakeClient::new(
        std::env::var("SNOWFLAKE_USERNAME")
            .expect("missing SNOWFLAKE_USERNAME env")
            .as_str(),
        SnowflakeAuthMethod::Password(
            std::env::var("SNOWFLAKE_PASSWORD")
                .expect("missing SNOWFLAKE_PASSWORD env")
                .to_string(),
        ),
        SnowflakeClientConfig {
            account: std::env::var("SNOWFLAKE_ACCOUNT")
                .expect("missing SNOWFLAKE_ACCOUNT env")
                .to_string(),
            role: Some(
                std::env::var("SNOWFLAKE_ROLE")
                    .expect("missing SNOWFLAKE_ROLE env")
                    .to_string(),
            ),
            warehouse: Some(
                std::env::var("SNOWFLAKE_WAREHOUSE")
                    .expect("missing SNOWFLAKE_WAREHOUSE env")
                    .to_string(),
            ),
            database: Some(
                std::env::var("SNOWFLAKE_DATABASE")
                    .expect("missing SNOWFLAKE_DATABASE env")
                    .to_string(),
            ),
            schema: Some(
                std::env::var("SNOWFLAKE_SCHEMA")
                    .expect("missing SNOWFLAKE_SCHEMA env")
                    .to_string(),
            ),
        },
    )?;

    Ok(client)
}
