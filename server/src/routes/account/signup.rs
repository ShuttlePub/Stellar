use application::transfer::account::{CreateAccountDto, CreateNonVerifiedAccountDto, NonVerifiedAccountDto};
use axum::{response::IntoResponse, http::StatusCode, extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use application::services::{DependOnCreateAccountService, DependOnCreateNonVerifiedAccountService};

use crate::Handler;

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
    State(handler): State<Handler>,
    verified: Option<Query<Ticket>>,
    Json(form): Json<UserInput>,
) -> Result<impl IntoResponse, StatusCode> {
    match verified {
        Some(Query(ticket)) => {
            let Some(name) = form.name else {
                return Err(StatusCode::BAD_REQUEST);
            };
            let Some(pass) = form.pass else {
                return Err(StatusCode::BAD_REQUEST);
            };
            let create = CreateAccountDto::new(name, pass);

            use application::services::CreateAccountService;
            let _account = handler.create_account_service()
                .create(&ticket.id, create).await
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

            use application::services::CreateNonVerifiedAccountService;
            let NonVerifiedAccountDto { id, .. } = handler.create_non_verified_account_service()
                .create(non_verified).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok((StatusCode::SEE_OTHER, Json(Response::new(id))))
        },
    }
}
