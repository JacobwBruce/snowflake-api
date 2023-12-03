use axum::http;
use axum::routing::{get, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Capacity {
    pub capacity: u32,
}

async fn get_capacity() -> Result<(http::StatusCode, axum::Json<Capacity>), http::StatusCode> {
    let capacity = Capacity { capacity: 42 };
    Ok((http::StatusCode::CREATED, axum::Json(capacity)))
}
pub fn capacity_router() -> Router {
    // Create a new router
    let mut router = Router::new();

    // Add the /capacity route to the router
    router = router.route("/", get(get_capacity));

    // Return the router
    router
}
