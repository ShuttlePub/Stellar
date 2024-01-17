use super::forms::RegistrationForm;
use crate::{Handler, ServerError};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::typed_header::TypedHeader;

pub async fn register(
    State(_handler): State<Handler>,
    TypedHeader(_bearer): TypedHeader<Authorization<Bearer>>,
    Json(_form): Json<RegistrationForm>,
) -> Result<impl IntoResponse, ServerError> {
    Ok(())
}
