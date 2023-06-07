use axum::{http::StatusCode, extract::State, response::IntoResponse};
use axum::extract::Query;
use application::services::{AcceptAuthorizeTokenService, DependOnAcceptAuthorizeTokenService, DependOnRejectAuthorizeTokenService, RejectAuthorizeTokenService};
use application::transfer::token::AcceptUserFormDto;
use crate::{Handler, ServerError};

use self::forms::*;

pub async fn accept(
    State(handler): State<Handler>,
    Query(query): Query<UserQueryAccept>
) -> Result<impl IntoResponse, ServerError> {
    let input = AcceptUserFormDto {
        address: "".to_string(),
        pass: "".to_string(),
    };
    handler
        .accept_authorize_token_service()
        .accept(&query.ticket, &query.state, input)
        .await?;
    Ok(StatusCode::OK)
}

pub async fn reject(
    State(handler): State<Handler>,
    Query(query): Query<UserQueryReject>
) -> Result<impl IntoResponse, ServerError> {
    handler
        .reject_authorize_token_service()
        .reject(&query.ticket)
        .await?;
    Ok(StatusCode::OK)
}


mod forms {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct UserQueryAccept {
        pub ticket: String,
        pub state: String
    }

    #[derive(Deserialize)]
    pub struct UserQueryReject {
        pub ticket: String
    }
}