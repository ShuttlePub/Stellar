use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::Handler;

pub async fn user_reject(
    State(_handler): State<Handler>
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::OK)
}