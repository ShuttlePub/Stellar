use axum::{http::StatusCode, extract::State, Json, response::IntoResponse};
use application::services::{DependOnRejectAuthorizeTokenService, RejectAuthorizeTokenService};
use crate::{Handler, ServerError};

use self::actions::Ticket;

pub async fn accept(
    State(_handler): State<Handler>
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::OK)
}

pub async fn reject(
    State(_handler): State<Handler>,
    Json(ticket): Json<Ticket>
) -> Result<impl IntoResponse, ServerError> {
    _handler
        .reject_authorize_token_service()
        .reject(&ticket.ticket_id)
        .await?;
    Ok(StatusCode::OK)
}


mod actions {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Ticket {
        pub ticket_id: String
    }
}