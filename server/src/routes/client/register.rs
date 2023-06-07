use axum::extract::State;
use axum::{Json, TypedHeader};
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::response::IntoResponse;
use crate::{Handler, ServerError};
use super::forms::RegistrationForm;

pub async fn register(
    State(_handler): State<Handler>,
    TypedHeader(_bearer): TypedHeader<Authorization<Bearer>>,
    Json(_form): Json<RegistrationForm>,
) -> Result<impl IntoResponse, ServerError> {
    Ok(())
}