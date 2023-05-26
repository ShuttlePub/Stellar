use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Response {
    client_id: String,
    client_id_iat: u64,
    client_secret: String,
    client_secret_exp: u64,
    response_code: Vec<String>,
    #[serde(rename = "token_endpoint_auth_method")]
    tepam: Vec<String>,
    grant_types: Vec<String>,
    response_types: Vec<String>,
    client_name: String,
    client_uri: String,
    logo_uri: String,
    scopes: Vec<String>,
    contacts: Vec<String>,
    tos_uri: String,
    policy_uri: String,
    jwks_uri: String, // ───┬─ MUST NOT both be present in the same request or response.
    jwks: String,     // ───┘
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "snake_case"))]
#[serde(tag = "error", content = "error_description")]
pub enum ErrorResponse {
    InvalidRedirectUri(String),
    InvalidClientMetadata(String),
    Unexpect(String)
}

impl ErrorResponse {
    pub fn un_expect(desc: impl Into<String>) -> Self {
        Self::Unexpect(desc.into())
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}
