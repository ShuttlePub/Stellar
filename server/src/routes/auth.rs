use axum::{response::IntoResponse, http::StatusCode, extract::{State, Query}};
use serde::Deserialize;

use crate::InteractionHandler;

pub async fn authorization(
    State(_handler): State<InteractionHandler>,
    Query(_query): Query<AuthorizationGrantQuery>
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct AuthorizationGrantQuery {
    response_type: String,
    client_id: String,
    redirect_uri: Option<String>,
    scope: Option<Vec<String>>,
    state: String,
    code_challenge: String,
    code_challenge_method: String
}