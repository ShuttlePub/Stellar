use crate::ServerError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::{headers::Cookie, typed_header::TypedHeader};

pub const SESSION_TAG: &str = "stellar_session";

pub enum Session {
    None,
    Code(String),
}

impl From<Session> for Option<String> {
    fn from(value: Session) -> Self {
        match value {
            Session::None => None,
            Session::Code(code) => Some(code),
        }
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Session
where
    S: Sync + Send,
{
    type Rejection = ServerError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookie: Option<TypedHeader<Cookie>> = parts.extract().await?;
        let session = cookie.as_ref().and_then(|cookie| cookie.get(SESSION_TAG));
        Ok(match session {
            None => Session::None,
            Some(code) => Session::Code(code.to_string()),
        })
    }
}
