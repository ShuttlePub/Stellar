use application::{ApplicationError, ExpectUserAction};
use axum::{
    http::header::CONTENT_LOCATION,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::headers::{HeaderValue, HeaderMap};
use driver::DriverError;
use serde_json::json;
use std::convert::Infallible;
use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    Application(ApplicationError),
    #[error(transparent)]
    Driver(#[from] DriverError),
    #[error("invalid value `{value}` in the following {method}.")]
    InvalidValue { method: &'static str, value: String },
    #[error(transparent)]
    Axum(anyhow::Error),
    #[error(transparent)]
    Serde(anyhow::Error),
    #[error(transparent)]
    RequestParse(anyhow::Error),
    #[error("require user action.")]
    RequireUserAction(ExpectUserAction),
    #[error(transparent)]
    Infallible(#[from] Infallible),
}

impl serde::de::Error for ServerError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        ServerError::Serde(anyhow::Error::msg(msg.to_string()))
    }
}

impl From<ApplicationError> for ServerError {
    fn from(value: ApplicationError) -> Self {
        match value {
            ApplicationError::RequireUserAction(expect) => Self::RequireUserAction(expect),
            _ => Self::Application(value),
        }
    }
}

impl From<kernel::external::UuidError> for ServerError {
    fn from(e: kernel::external::UuidError) -> Self {
        Self::RequestParse(anyhow::Error::new(e))
    }
}

impl From<axum::Error> for ServerError {
    fn from(e: axum::Error) -> Self {
        Self::Axum(anyhow::Error::new(e))
    }
}

impl From<axum_extra::headers::Error> for ServerError {
    fn from(e: axum_extra::headers::Error) -> Self {
        Self::Axum(anyhow::Error::new(e))
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let msg = match self {
            ServerError::Axum(e) => e.to_string(),
            ServerError::Driver(e) => e.to_string(),
            ServerError::InvalidValue { method, value } => {
                format!("invalid value `{value}` in the following {method}.")
            }
            ServerError::Serde(e) => e.to_string(),
            ServerError::RequestParse(e) => e.to_string(),
            ServerError::Application(e) => e.to_string(),
            ServerError::Infallible(e) => e.to_string(),
            ServerError::RequireUserAction(expect) => {
                return require_user_actions(expect).into_response()
            }
        };

        let json = json!({ "error": msg });

        (StatusCode::BAD_REQUEST, Json(json)).into_response()
    }
}

fn require_user_actions(expect: ExpectUserAction) -> impl IntoResponse {
    match expect {
        ExpectUserAction::Login => (
            StatusCode::FORBIDDEN,
            HeaderMap::new(),
            "session expired. please re-login.",
        ),
        ExpectUserAction::MFA => {
            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_LOCATION,
                HeaderValue::from_static("/accounts/verify"),
            );
            (StatusCode::ACCEPTED, headers, "accepted login process, We have sent you an email including an authentication code, please add the code to the form and continue the process.")
        }
    }
}
