use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::KernelError;

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub enum TokenEndPointAuthMethod {
    ClientSecretPost,
    ClientSecretBasic,
    None,

    PrivateKeyJWK
}

impl TryFrom<String> for TokenEndPointAuthMethod {
    type Error = KernelError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        TokenEndPointAuthMethod::from_str(value.as_str())
    }
}

impl FromStr for TokenEndPointAuthMethod {
    type Err = KernelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "client_secret_post" => Ok(Self::ClientSecretPost),
            "client_secret_basic" => Ok(Self::ClientSecretBasic),
            "none" => Ok(Self::None),
            "private_key_jwk" => Ok(Self::PrivateKeyJWK),
            _ => Err(KernelError::InvalidValue {
                method: "from_str",
                value: s.to_string(),
            })
        }
    }
}

impl AsRef<str> for TokenEndPointAuthMethod {
    fn as_ref(&self) -> &str {
        match self {
            TokenEndPointAuthMethod::ClientSecretPost => "client_secret_post",
            TokenEndPointAuthMethod::ClientSecretBasic => "client_secret_basic",
            TokenEndPointAuthMethod::None => "none",
            TokenEndPointAuthMethod::PrivateKeyJWK => "private_key_jwk"
        }
    }
}