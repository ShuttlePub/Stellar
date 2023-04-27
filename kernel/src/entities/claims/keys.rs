use std::str::FromStr;
use jsonwebkey::JsonWebKey;
use serde::{Deserialize, Serialize};
use url::Url;
use crate::KernelError;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Jwks {
    Uri(String),
    Key(JsonWebKey)
}

impl Jwks {
    pub fn new(resource: impl Into<String>) -> Result<Self, KernelError> {
        let resource = resource.into();
        match JsonWebKey::from_str(&resource) {
            Ok(key) => Ok(Self::Key(key)),
            Err(jwke) => match Url::from_str(&resource) {
                Ok(url) => Ok(Self::Uri(url.to_string())),
                Err(urle) => Err(KernelError::InvalidValue {
                    method: "parse key and url",
                    value: format!("jwk: {:?}, url: {:?}", jwke, urle),
                })
            }
        }
    }

    pub fn is_uri(&self) -> bool {
        match self {
            Jwks::Uri(_) => true,
            Jwks::Key(_) => false
        }
    }
}

impl TryFrom<Jwks> for String {
    type Error = KernelError;
    fn try_from(value: Jwks) -> Result<Self, Self::Error> {
        match value {
            Jwks::Uri(url) => Ok(url),
            Jwks::Key(_) => Err(KernelError::InvalidValue {
                method: "try convert to url string",
                value: "this value is jwk object.".to_string(),
            })
        }
    }
}

impl TryFrom<Jwks> for JsonWebKey {
    type Error = KernelError;
    fn try_from(value: Jwks) -> Result<Self, Self::Error> {
        match value {
            Jwks::Key(key) => Ok(key),
            Jwks::Uri(_) => Err(KernelError::InvalidValue {
                method: "try convert to jwk",
                value: "this value is uri.".to_string(),
            })
        }
    }
}