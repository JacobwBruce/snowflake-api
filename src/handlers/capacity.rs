use axum::routing::{get, patch, Router};
use axum::{extract, http};

use crate::models::capacity::{Capacity, NewCapacity};
use crate::repositories::capacity_repo::CapacityRepository;

async fn get_capacity(
    extract::State(db): extract::State<CapacityRepository>,
) -> Result<(http::StatusCode, axum::Json<Vec<Capacity>>), http::StatusCode> {
    let res = db.get_capacity().await;
    match res {
        Ok(capacity) => Ok((http::StatusCode::OK, axum::Json(capacity))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_capacity(
    extract::State(db): extract::State<CapacityRepository>,
    extract::Path(id): extract::Path<i32>,
    axum::Json(payload): axum::Json<NewCapacity>,
) -> http::StatusCode {
    let res = db
        .update_capacity(id, payload)
        .await
        .map(|res| match res.rows_affected() {
            0 => http::StatusCode::NOT_FOUND,
            _ => http::StatusCode::NO_CONTENT,
        })
        .unwrap_or_else(|_| http::StatusCode::INTERNAL_SERVER_ERROR);
    res
}

pub fn capacity_router<S>(db: &CapacityRepository) -> Router<S> {
    Router::new()
        .route("/", get(get_capacity))
        .route("/:id", patch(update_capacity))
        .with_state(db.clone())
}
