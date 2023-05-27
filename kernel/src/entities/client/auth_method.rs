use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::KernelError;

#[derive(Debug, Clone, Hash)]
pub enum TokenEndPointAuthMethod {
    ClientSecretPost,
    ClientSecretBasic,
    None,

    PrivateKeyJWT
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
            "private_key_jwt" => Ok(Self::PrivateKeyJWT),
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
            TokenEndPointAuthMethod::PrivateKeyJWT => "private_key_jwt"
        }
    }
}

impl From<TokenEndPointAuthMethod> for String {
    fn from(value: TokenEndPointAuthMethod) -> Self {
        value.as_ref().to_string()
    }
}

impl<'de> Deserialize<'de> for TokenEndPointAuthMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let deserialize: &str = Deserialize::deserialize(deserializer)?;
        Self::from_str(deserialize)
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for TokenEndPointAuthMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.as_ref())
    }
}