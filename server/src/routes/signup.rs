use application::{adapter::rest::RestAdapter, transfer::account::{CreateAccountDto, CreateNonVerifiedAccountDto, NonVerifiedAccountDto}};
use axum::{response::IntoResponse, http::StatusCode, extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};

use crate::InteractionHandler;

#[derive(Deserialize, Debug)]
pub struct Ticket {
    id: String
}

#[derive(Deserialize, Debug)]
pub struct UserInput {
    address: Option<String>,
    name: Option<String>,
    pass: Option<String>
}

#[derive(Serialize, Debug)]
pub enum Response {
    Ticket(String),
    Empty
}

impl Response {
    fn new(ticket: impl Into<String>) -> Self {
        Self::Ticket(ticket.into())
    }

    fn emptiness() -> Self {
        Self::Empty
    }
}

pub async fn signup(
    State(handler): State<InteractionHandler>,
    verified: Option<Query<Ticket>>,
    Json(form): Json<UserInput>,
) -> Result<impl IntoResponse, StatusCode> {
    let handler = handler.as_ref();
    match verified {
        Some(Query(ticket)) => {
            let Some(name) = form.name else {
                return Err(StatusCode::BAD_REQUEST);
            };
            let Some(pass) = form.pass else {
                return Err(StatusCode::BAD_REQUEST);
            };
            let create = CreateAccountDto::new(name, pass);
            let _account = handler.create_account(&ticket.id, create).await
                .map_err(|e| {
                    tracing::error!("{:#?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            Ok((StatusCode::CREATED, Json(Response::emptiness())))
        },
        None => {
            let Some(address) = form.address else {
                return Err(StatusCode::BAD_REQUEST);
            };
            let non_verified = CreateNonVerifiedAccountDto::new(address);
            let NonVerifiedAccountDto { id, .. } = handler
                .prepare_user_verification(non_verified).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            Ok((StatusCode::SEE_OTHER, Json(Response::new(id))))
        },
    }
}
