use crate::{Handler, ServerError};
use application::services::{DependOnPendingAuthorizeTokenService, PendingAuthorizeTokenService};
use application::transfer::token::CreateAuthorizeTokenDto;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use kernel::external::Uuid;
use serde::{Deserialize, Deserializer};

pub async fn authorization(
    State(handler): State<Handler>,
    Query(query): Query<AuthorizationGrantQuery>,
) -> Result<impl IntoResponse, ServerError> {
    let AuthorizationGrantQuery {
        response_type,
        client_id,
        redirect_uri,
        scope,
        state,
        code_challenge,
        code_challenge_method,
    } = query;

    let client_id = Uuid::parse_str(&client_id)?;

    let ticket = handler
        .pending_authorize_token_service()
        .pending(CreateAuthorizeTokenDto {
            response_type,
            client_id,
            redirect_uri,
            scope,
            state,
            code_challenge,
            code_challenge_method,
        })
        .await?;
    let value = serde_json::json!({
        "ticket": ticket.0
    });
    Ok(Json(value))
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct AuthorizationGrantQuery {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: Option<String>,
    #[serde(deserialize_with = "scope_deserializer")]
    pub scope: Vec<String>,
    pub state: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
}

/// This function converts a space-delimited string into an array.
///
/// Defined in [RFC6749 Section 3.3 Access Token Scope](https://datatracker.ietf.org/doc/html/rfc6749#section-3.3)
fn scope_deserializer<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: String = Deserialize::deserialize(deserializer)?;
    println!("{:?}", raw);
    let scopes = raw
        .split(' ')
        .map(|scope| scope.to_string())
        .collect::<Vec<String>>();
    Ok(scopes)
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct TestDomain {
        id: u64,
        #[serde(deserialize_with = "super::scope_deserializer")]
        array: Vec<String>,
    }

    #[test]
    fn space_separated_str_to_array_deserialize() -> anyhow::Result<()> {
        let json = r#"{
            "id": 10,
            "array": "read write external"
        }"#;
        let d: TestDomain = serde_json::from_str(json)?;

        println!("{:#?}", d);
        Ok(())
    }
}
