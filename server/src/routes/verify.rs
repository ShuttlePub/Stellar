use application::adaptor::rest::RestAdaptor;
use axum::{response::IntoResponse, http::StatusCode, extract::{Path, State}, Json};
use serde::{Deserialize, Serialize};

use crate::InteractionHandler;

#[derive(Deserialize, Debug)]
pub struct UserInput {
    code: String
}

#[derive(Serialize, Debug)]
pub struct Response {
    ticket: String
}

pub async fn verify(
    State(handler): State<InteractionHandler>,
    Path(id): Path<String>,
    Json(form): Json<UserInput>,
) -> Result<impl IntoResponse, StatusCode> {
    let valid = handler.as_ref().verify_account(&id, &form.code).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let res = Response {
        ticket: valid
    };
    Ok((StatusCode::TEMPORARY_REDIRECT, Json(res)))
}