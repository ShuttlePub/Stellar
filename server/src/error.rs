use std::fmt::Display;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use application::ApplicationError;
use driver::DriverError;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    Application(#[from] ApplicationError),
    #[error(transparent)]
    Driver(#[from] DriverError),
    #[error("invalid value `{value}` in the following {method}.")]
    InvalidValue {
        method: &'static str,
        value: String
    },
    #[error(transparent)]
    Serde(anyhow::Error),
    #[error(transparent)]
    RequestParse(anyhow::Error)
}

impl serde::de::Error for ServerError {
    fn custom<T>(msg: T) -> Self where T: Display {
        ServerError::Serde(anyhow::Error::msg(msg.to_string()))
    }
}

impl From<kernel::external::UuidError> for ServerError {
    fn from(e: kernel::external::UuidError) -> Self {
        Self::RequestParse(anyhow::Error::new(e))
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let msg = match self {
            ServerError::Driver(e)
                => e.to_string(),
            ServerError::InvalidValue { method, value }
                => format!("invalid value `{value}` in the following {method}."),
            ServerError::Serde(e)
                => e.to_string(),
            ServerError::RequestParse(e)
                => e.to_string(),
            ServerError::Application(e)
                => e.to_string()
        };

        let json = json!({
            "error": msg
        });

        (StatusCode::BAD_REQUEST, Json(json)).into_response()
    }
}