use std::collections::HashSet;
use serde::{Deserialize, Serialize};

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