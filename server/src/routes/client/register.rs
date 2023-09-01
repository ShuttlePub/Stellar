use super::forms::RegistrationForm;
use crate::{Handler, ServerError};
use axum::extract::State;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::response::IntoResponse;
use axum::{Json, TypedHeader};

pub async fn register(
    State(_handler): State<Handler>,
    TypedHeader(_bearer): TypedHeader<Authorization<Bearer>>,
    Json(_form): Json<RegistrationForm>,
) -> Result<impl IntoResponse, ServerError> {
    Ok(())
}
