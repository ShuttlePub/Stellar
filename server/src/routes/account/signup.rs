use application::{
    services::{DependOnCreateAccountService, DependOnCreateNonVerifiedAccountService},
    transfer::{
        account::{CreateAccountDto, CreateTemporaryAccountDto},
        mfa_code::TicketIdDto,
    },
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use self::forms::*;
use crate::Handler;

pub async fn signup(
    State(handler): State<Handler>,
    ticket: Option<Query<Ticket>>,
    Json(form): Json<UserInput>,
) -> Result<impl IntoResponse, StatusCode> {
    match ticket {
        Some(Query(query)) => {
            let Some(name) = form.name else {
                return Err(StatusCode::BAD_REQUEST);
            };
            let Some(pass) = form.pass else {
                return Err(StatusCode::BAD_REQUEST);
            };
            let create = CreateAccountDto::new(name, pass);

            use application::services::CreateAccountService;
            let _account = handler
                .create_account_service()
                .create(&query.ticket, create)
                .await
                .map_err(|e| {
                    tracing::error!("{:#?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            Ok((StatusCode::CREATED, Json(Response::new(None))))
        }
        None => {
            let Some(address) = form.address else {
                return Err(StatusCode::BAD_REQUEST);
            };
            let non_verified = CreateTemporaryAccountDto::new(address);

            use application::services::CreateTemporaryAccountService;
            let TicketIdDto(id) = handler
                .create_non_verified_account_service()
                .create(non_verified)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok((StatusCode::SEE_OTHER, Json(Response::new(id))))
        }
    }
}

mod forms {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Debug)]
    pub struct Ticket {
        pub ticket: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct UserInput {
        pub address: Option<String>,
        pub name: Option<String>,
        pub pass: Option<String>,
    }

    #[derive(Serialize)]
    pub struct Response {
        #[serde(skip_serializing_if = "Option::is_none")]
        ticket: Option<String>,
    }

    impl Response {
        pub fn new(ticket: impl Into<Option<String>>) -> Self {
            Self {
                ticket: ticket.into(),
            }
        }
    }
}
