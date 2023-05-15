use std::fmt::Display;
use driver::DriverError;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    Driver(#[from] DriverError),
    #[error("invalid value `{value}` in the following {method}.")]
    InvalidValue {
        method: &'static str,
        value: String
    },
    #[error(transparent)]
    Serde(anyhow::Error)
}

impl serde::de::Error for ServerError {
    fn custom<T>(msg: T) -> Self where T: Display {
        ServerError::Serde(anyhow::Error::msg(msg.to_string()))
    }
}