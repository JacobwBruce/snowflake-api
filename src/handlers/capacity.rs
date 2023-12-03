use axum::http;
use axum::routing::{get, Router};

use crate::models::capacity::Capacity;

async fn get_capacity() -> Result<(http::StatusCode, axum::Json<Capacity>), http::StatusCode> {
    let capacity = Capacity { capacity: 42 };
    Ok((http::StatusCode::CREATED, axum::Json(capacity)))
}

pub fn capacity_router() -> Router {
    let mut router = Router::new();

    router = router.route("/", get(get_capacity));

    router
}
