use serde::{Serialize, Deserialize};
use try_ref::TryAsRef;
use crate::KernelError;
use super::ClientSecret;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum ClientTypes {
    Confidential(ClientSecret),
    Public
}

impl ClientTypes {
    pub fn new(secret: impl Into<Option<ClientSecret>>) -> Self {
        match secret.into() {
            Some(secret) => Self::Confidential(secret),
            None => Self::Public
        }
    }
}

impl From<ClientSecret> for ClientTypes {
    fn from(value: ClientSecret) -> Self {
        Self::Confidential(value)
    }
}

impl From<ClientTypes> for Option<ClientSecret> {
    fn from(origin: ClientTypes) -> Self {
        match origin {
            ClientTypes::Confidential(secret) => Some(secret),
            ClientTypes::Public => None,
        }
    }
}

impl TryAsRef<ClientSecret> for ClientTypes {
    type Error = KernelError;
    fn try_as_ref(&self) -> Result<&ClientSecret, Self::Error> {
        match self {
            ClientTypes::Confidential(secret) => { Ok(secret) },
            ClientTypes::Public => Err(KernelError::InvalidValue {
                method: "try_as_ref",
                value: "client secret".to_string(),
            })
        }
    }
}

impl Default for ClientTypes {
    fn default() -> Self {
        Self::Public
    }
}