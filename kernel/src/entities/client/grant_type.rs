use std::collections::HashSet;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::KernelError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrantTypes(HashSet<GrantType>);

impl GrantTypes {
    pub fn new(types: impl Into<Vec<GrantType>>) -> Self {
        Self(types.into().into_iter().collect())
    }

    pub fn iter(&self) -> impl Iterator<Item = &GrantType> {
        self.0.iter()
    }
}

impl From<GrantTypes> for Vec<GrantType> {
    fn from(values: GrantTypes) -> Self {
        values.into_iter().collect()
    }
}

impl IntoIterator for GrantTypes {
    type Item = GrantType;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<GrantType> for GrantTypes {
    fn from_iter<T: IntoIterator<Item = GrantType>>(iter: T) -> Self {
        let v = iter.into_iter()
            .collect::<Vec<GrantType>>();
        Self::new(v)
    }
}

impl From<GrantTypes> for HashSet<GrantType> {
    fn from(value: GrantTypes) -> Self {
        value.0
    }
}

impl AsRef<HashSet<GrantType>> for GrantTypes {
    fn as_ref(&self) -> &HashSet<GrantType> {
        &self.0
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum GrantType {
    AuthorizationCode,
    Implicit,
    Password,
    ClientCredentials,
    RefreshToken,
    JWTBearer,
    Saml2Bearer
}

impl TryFrom<String> for GrantType {
    type Error = KernelError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        GrantType::from_str(value.as_str())
    }
}

impl FromStr for GrantType {
    type Err = KernelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "authorization_code" => Self::AuthorizationCode,
            "implicit" => Self::Implicit,
            "password" => Self::Password,
            "client_credentials" => Self::ClientCredentials,
            "refresh_token" => Self::RefreshToken,
            "jwt_bearer" => Self::JWTBearer,
            "saml2_bearer" => Self::Saml2Bearer,
            _ => return Err(KernelError::InvalidValue {
                method: "from_str",
                value: s.to_string(),
            })
        })
    }
}

impl AsRef<str> for GrantType {
    fn as_ref(&self) -> &str {
        match self {
            GrantType::AuthorizationCode => "authorization_code",
            GrantType::Implicit => "implicit",
            GrantType::Password => "password",
            GrantType::ClientCredentials => "client_credentials",
            GrantType::RefreshToken => "refresh_token",
            GrantType::JWTBearer => "jwt_bearer",
            GrantType::Saml2Bearer => "saml2_bearer"
        }
    }
}

impl From<GrantType> for String {
    fn from(value: GrantType) -> Self {
        value.as_ref().to_string()
    }
}