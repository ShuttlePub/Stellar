use axum::{response::IntoResponse, http::StatusCode, extract::State, Json};
use serde::Deserialize;

use crate::InteractionHandler;

pub async fn client_registration(
    State(_handler): State<InteractionHandler>,
    Json(_form): Json<ClientRegistrationForm>
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct ClientRegistrationForm {
    name: String,
    display_name: String,
    owner: String,
    description: String,
    scopes: Vec<ClientScopeRegistration>,
    secret: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct ClientScopeRegistration {
    method: String,
    description: String
}