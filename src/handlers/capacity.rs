use axum::routing::{delete, get, patch, post, Router};
use axum::{extract, http};

use crate::models::capacity::{Capacity, NewCapacity};
use crate::repositories::capacity_repo::CapacityRepository;

#[derive(serde::Serialize)]
struct ErrorResponse {
    message: String,
}

async fn get_capacity(
    extract::State(db): extract::State<CapacityRepository>,
) -> Result<
    (http::StatusCode, axum::Json<Vec<Capacity>>),
    (http::StatusCode, axum::Json<ErrorResponse>),
> {
    let res = db.get_capacity().await;
    match res {
        Ok(capacity) => Ok((http::StatusCode::OK, axum::Json(capacity))),
        Err(err) => Err((
            http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(ErrorResponse {
                message: err.to_string(),
            }),
        )),
    }
}

async fn get_capacity_by_id(
    extract::State(db): extract::State<CapacityRepository>,
    extract::Path(id): extract::Path<i32>,
) -> Result<(http::StatusCode, axum::Json<Capacity>), (http::StatusCode, axum::Json<ErrorResponse>)>
{
    let res = db.get_capacity_by_id(id).await;
    match res {
        Ok(capacity) => Ok((http::StatusCode::OK, axum::Json(capacity))),
        Err(err) => {
            if let sqlx::Error::RowNotFound = err {
                Err((
                    http::StatusCode::NOT_FOUND,
                    axum::Json(ErrorResponse {
                        message: "No capacity found for id: ".to_string() + &id.to_string(),
                    }),
                ))
            } else {
                Err((
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(ErrorResponse {
                        message: err.to_string(),
                    }),
                ))
            }
        }
    }
}

async fn create_capacity(
    extract::State(db): extract::State<CapacityRepository>,
    axum::Json(payload): axum::Json<NewCapacity>,
) -> Result<http::StatusCode, (http::StatusCode, axum::Json<ErrorResponse>)> {
    let res = db.create_capacity(payload).await;
    match res {
        Ok(_) => Ok(http::StatusCode::CREATED),
        Err(err) => Err((
            http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(ErrorResponse {
                message: err.to_string(),
            }),
        )),
    }
}

async fn update_capacity(
    extract::State(db): extract::State<CapacityRepository>,
    extract::Path(id): extract::Path<i32>,
    axum::Json(payload): axum::Json<NewCapacity>,
) -> Result<http::StatusCode, (http::StatusCode, axum::Json<ErrorResponse>)> {
    let res = db.update_capacity(id, payload).await;
    tracing::info!("Update capacity: {}", id);

    match res {
        Ok(sql_result) => {
            if sql_result.rows_affected() == 0 {
                Err((
                    http::StatusCode::NOT_FOUND,
                    axum::Json(ErrorResponse {
                        message: "No capacity found for id: ".to_string() + &id.to_string(),
                    }),
                ))
            } else {
                Ok(http::StatusCode::NO_CONTENT)
            }
        }
        Err(err) => Err((
            http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(ErrorResponse {
                message: err.to_string(),
            }),
        )),
    }
}

async fn delete_capacity(
    extract::State(db): extract::State<CapacityRepository>,
    extract::Path(id): extract::Path<i32>,
) -> Result<http::StatusCode, (http::StatusCode, axum::Json<ErrorResponse>)> {
    let res = db.delete_capacity(id).await;
    match res {
        Ok(sql_result) => {
            if sql_result.rows_affected() == 0 {
                Err((
                    http::StatusCode::NOT_FOUND,
                    axum::Json(ErrorResponse {
                        message: "No capacity found for id: ".to_string() + &id.to_string(),
                    }),
                ))
            } else {
                Ok(http::StatusCode::NO_CONTENT)
            }
        }
        Err(err) => Err((
            http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(ErrorResponse {
                message: err.to_string(),
            }),
        )),
    }
}

pub fn capacity_router<S>(db: &CapacityRepository) -> Router<S> {
    Router::new()
        .route("/", get(get_capacity))
        .route("/:id", get(get_capacity_by_id))
        .route("/", post(create_capacity))
        .route("/:id", patch(update_capacity))
        .route("/:id", delete(delete_capacity))
        .with_state(db.clone())
}
