use application::{adaptor::rest::RestAdaptor, transfer::account::CreateAccountDto};
use axum::{response::IntoResponse, http::StatusCode, extract::{Path, State}, Json};
use serde::Deserialize;

use crate::InteractionHandler;

pub mod prepare {
    use application::{adaptor::rest::RestAdaptor, transfer::account::{CreateNonVerifiedAccountDto, NonVerifiedAccountDto}};
    use axum::{response::IntoResponse, http::StatusCode, Json, extract::State};
    use serde::{Deserialize, Serialize};

    use crate::InteractionHandler;

    #[derive(Deserialize, Debug)]
    pub struct UserInput {
        pub address: String
    }

    #[derive(Serialize, Debug)]
    pub struct Response {
        ticket: String,
        end_point: String
    }

    pub async fn signup_prepare(
        State(handler): State<InteractionHandler>,
        Json(form): Json<UserInput>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let user = CreateNonVerifiedAccountDto::new(form.address);
        let NonVerifiedAccountDto { id, ..} = handler.as_ref().prepare_user_verification(user).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let res = Response {
            ticket: id,
            end_point: "/verify/:id".to_owned()
        };
        Ok((StatusCode::SEE_OTHER, Json(res)))
    }
}

#[derive(Deserialize, Debug)]
pub struct UserInput {
    name: String,
    pass: String
}

pub async fn signup(
    State(handler): State<InteractionHandler>,
    Path(ticket): Path<String>,
    Json(form): Json<UserInput>,
) -> Result<impl IntoResponse, StatusCode> {
    let create = CreateAccountDto::new(form.name, form.pass);
    handler.as_ref().create_account(&ticket, create).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::CREATED)
}