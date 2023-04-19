use serde::{Serialize, Deserialize};

use super::ClientSecret;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum ClientTypes {
    Confidential(ClientSecret),
    Public
}

impl ClientTypes {
    pub fn new(secret: impl Into<Option<String>>) -> Self {
        match secret.into() {
            Some(secret) => Self::Confidential(ClientSecret::new(secret)),
            None => Self::Public
        }
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

impl From<ClientTypes> for Option<String> {
    fn from(value: ClientTypes) -> Self {
        match value {
            ClientTypes::Confidential(secret) => Some(secret.into()),
            ClientTypes::Public => None,
        }
    }
}