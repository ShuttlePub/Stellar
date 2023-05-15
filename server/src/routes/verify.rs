use axum::{response::IntoResponse, http::StatusCode, extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use application::services::{ApproveAccountService, DependOnApproveAccountService};

use crate::Handler;

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
    State(handler): State<Handler>,
    Query(ticket): Query<Ticket>,
    Json(form): Json<UserInput>,
) -> Result<impl IntoResponse, StatusCode> {
    let valid = handler.approve_account_service()
        .approve(&ticket.id, &form.code)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let res = Response {
        ticket: valid
    };
    Ok((StatusCode::TEMPORARY_REDIRECT, Json(res)))
}