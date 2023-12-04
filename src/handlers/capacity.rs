use axum::routing::{get, patch, Router};
use axum::{extract, http};
use sqlx::MySqlPool;

use crate::models::capacity::{Capacity, NewCapacity};
use crate::AppState;

async fn get_capacity(
    extract::State(pool): extract::State<MySqlPool>,
) -> Result<(http::StatusCode, axum::Json<Vec<Capacity>>), http::StatusCode> {
    let res = sqlx::query_as::<_, Capacity>("SELECT * FROM capacity")
        .fetch_all(&pool)
        .await;
    match res {
        Ok(capacity) => Ok((http::StatusCode::OK, axum::Json(capacity))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_capacity(
    extract::State(pool): extract::State<MySqlPool>,
    extract::Path(id): extract::Path<i32>,
    axum::Json(payload): axum::Json<NewCapacity>,
) -> http::StatusCode {
    let res = sqlx::query(
        r#"
        UPDATE capacity
        SET name = ?,
            location = ?,
            num_of_vendors_needed = ?,
            tsa_needed = ?
        WHERE id = ?
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.location)
    .bind(&payload.num_of_vendors_needed)
    .bind(&payload.tsa_needed)
    .bind(&id)
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => http::StatusCode::NOT_FOUND,
        _ => http::StatusCode::NO_CONTENT,
    });

    match res {
        Ok(status) => status,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub fn capacity_router<S>(state: &AppState) -> Router<S> {
    Router::new()
        .route("/", get(get_capacity))
        .route("/:id", patch(update_capacity))
        .with_state(state.pool.clone())
}
