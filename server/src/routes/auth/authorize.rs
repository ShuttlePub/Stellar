use axum::{extract::{Query, State}, http::StatusCode, Json, response::IntoResponse};
use serde::{Deserialize, Deserializer};
use crate::Handler;

pub async fn authorization(
    State(_handler): State<Handler>,
    Query(_query): Query<AuthorizationGrantQuery>
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

pub async fn user_decision(
    State(_handler): State<Handler>
) {

}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct AuthorizationGrantQuery {
    response_type: String,
    client_id: String,
    redirect_uri: Option<String>,
    #[serde(deserialize_with = "scope_deserializer")]
    scope: Vec<String>,
    state: String,
    code_challenge: String,
    code_challenge_method: String
}

/// This function converts a space-delimited string into an array.
///
/// Defined in [RFC6749 Section 3.3 Access Token Scope](https://datatracker.ietf.org/doc/html/rfc6749#section-3.3)
fn scope_deserializer<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
  where D: Deserializer<'de>
{
    let raw: &str = Deserialize::deserialize(deserializer)?;
    let scopes = raw.split(' ')
        .map(ToOwned::to_owned)
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
        array: Vec<String>
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