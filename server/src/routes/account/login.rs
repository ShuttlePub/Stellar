use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::{Handler, ServerError};

pub async fn login(
    State(_handler): State<Handler>
) -> Result<impl IntoResponse, ServerError> {
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

