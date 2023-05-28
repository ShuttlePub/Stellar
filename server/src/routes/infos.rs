use std::collections::HashMap;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use kernel::entities::ClientName;
use kernel::external::{JsonWebKey, Uuid};
use kernel::repository::{ClientRegistry, DependOnClientRegistry};
use crate::Handler;

pub async fn stellar_info(
    State(handler): State<Handler>
) -> Result<impl IntoResponse, StatusCode> {
    let stellar = handler
        .client_registry()
        .find_by_name(&ClientName::new("Stellar"))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let stellar = stellar
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let stellar = stellar.into_destruct();
    let stellar = StellarClient {
        client_id: stellar.id.into(),
        client_name: stellar.name.into(),
        client_uri: stellar.uri.into(),
        description: stellar.desc.into(),
        logo_uri: stellar.logo.into(),
        tos_uri: stellar.terms.into(),
        owner_id: stellar.owner.into(),
        policy_uri: stellar.policy.into(),
        tep_am: stellar.auth_method.into(),
        grant_types: stellar.grant_types
            .into_iter()
            .map(Into::into)
            .collect(),
        response_types: stellar.response_types
            .into_iter()
            .map(Into::into)
            .collect(),
        redirect_uris: stellar.redirect_uris
            .into_iter()
            .map(Into::into)
            .collect(),
        scopes: stellar.scopes.into(),
        contacts: stellar.contact
            .into_iter()
            .map(Into::into)
            .collect(),
        jwks: stellar.jwks.clone()
            .filter(|key| !key.is_uri())
            .map(TryInto::<JsonWebKey>::try_into)
            .transpose()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .map(|key| key.to_string()),
        jwks_uri: stellar.jwks.filter(|key| key.is_uri())
            .map(TryInto::<String>::try_into)
            .transpose()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };
    Ok(Json(stellar))
}

#[derive(Serialize)]
struct StellarClient {
    client_id: Uuid,
    client_name: String,
    client_uri: String,
    description: String,
    logo_uri: String,
    tos_uri: String,
    owner_id: Uuid,
    policy_uri: String,
    #[serde(rename = "token_endpoint_auth_method")]
    tep_am: String,
    grant_types: Vec<String>,
    response_types: Vec<String>,
    redirect_uris: Vec<String>,
    scopes: HashMap<String, Option<String>>,
    contacts: Vec<String>,
    jwks: Option<String>,
    jwks_uri: Option<String>
}