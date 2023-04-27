use std::str::FromStr;
use jsonwebkey::JsonWebKey;
use url::Url;
use crate::KernelError;

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
}

