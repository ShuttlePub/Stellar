use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrantTypes(HashSet<GrantType>);

impl GrantTypes {
    pub fn new(types: impl Into<Vec<GrantType>>) -> Self {
        Self(types.into().into_iter().collect())
    }
}

impl From<GrantTypes> for Vec<GrantType> {
    fn from(value: GrantTypes) -> Self {
        value.0.into_iter().collect()
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