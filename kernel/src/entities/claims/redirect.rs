use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedirectUris(HashSet<RedirectUri>);

impl RedirectUris {
    pub fn new(uris: impl Into<Vec<RedirectUri>>) -> Self {
        Self(uris.into().into_iter().collect())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct RedirectUri(String);

impl RedirectUri {
    pub fn new(uri: impl Into<String>) -> Self {
        Self(uri.into())
    }
}

impl From<RedirectUri> for String {
    fn from(origin: RedirectUri) -> Self {
        origin.0
    }
}

impl AsRef<str> for RedirectUri {
    fn as_ref(&self) -> &str {
        &self.0
    }
}