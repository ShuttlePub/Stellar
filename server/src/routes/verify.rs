use application::adapter::rest::RestAdapter;
use axum::{response::IntoResponse, http::StatusCode, extract::{State, Query}, Json};
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

#[derive(Deserialize, Debug)]
pub struct Ticket {
    id: String
}

pub async fn verify(
    State(handler): State<InteractionHandler>,
    Query(ticket): Query<Ticket>,
    Json(form): Json<UserInput>,
) -> Result<impl IntoResponse, StatusCode> {
    let valid = handler.as_ref()
        .approve_account(&ticket.id, &form.code)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let res = Response {
        ticket: valid
    };
    Ok((StatusCode::TEMPORARY_REDIRECT, Json(res)))
}