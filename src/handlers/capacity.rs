use axum::routing::{get, patch, Router};
use axum::{extract, http};

use crate::models::capacity::Capacity;

async fn get_capacity() -> Result<(http::StatusCode, axum::Json<Capacity>), http::StatusCode> {
    let capacity = Capacity { capacity: 42 };
    Ok((http::StatusCode::OK, axum::Json(capacity)))
}

async fn update_capacity(
    extract::Path(id): extract::Path<i32>,
    axum::Json(payload): axum::Json<Capacity>,
) -> http::StatusCode {
    tracing::info!("Updating capacity with id {}", id);
    tracing::info!("Payload: {:?}", payload);
    http::StatusCode::NO_CONTENT
}

pub fn capacity_router() -> Router {
    let mut router = Router::new();

    router = router.route("/", get(get_capacity));
    router = router.route("/:id", patch(update_capacity));

    router
}
