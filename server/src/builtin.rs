#![allow(unused)]

use application::transfer::client::{RegisterClientDto, TokenEndPointAuthMethodDto};
use crate::{Handler, ServerError};

fn initialize(handler: Handler) -> Result<(), ServerError> {
    let client_uri = dotenvy::var("CLIENT_URI")
        .map_err(|e| ServerError::InvalidValue {
            method: "initialize",
            value: e.to_string(),
        })?;

    let self_reg = RegisterClientDto {
        name: "Stellar".to_string(),
        client_uri,
        description: "".to_string(),
        logo_uri: "".to_string(),
        tos_uri: "".to_string(),
        owner_id: Default::default(),
        policy_uri: "".to_string(),
        auth_method: TokenEndPointAuthMethodDto::PrivateKeyJWT,
        grant_types: vec![],
        response_types: vec![],
        redirect_uris: vec![],
        scopes: vec![],
        contacts: vec![],
        jwk: None,
    };

    Ok(())
}