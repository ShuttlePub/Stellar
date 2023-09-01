use crate::KernelError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedirectUris(HashSet<RedirectUri>);

impl RedirectUris {
    pub fn new(uris: impl Into<Vec<RedirectUri>>) -> Self {
        Self(uris.into().into_iter().collect())
    }

    pub fn iter(&self) -> impl Iterator<Item = &RedirectUri> {
        self.0.iter()
    }

    pub fn is_many(&self) -> bool {
        self.0.len() > 1
    }

    pub fn take_one(self) -> Result<RedirectUri, KernelError> {
        if self.is_many() {
            return Err(KernelError::InvalidValue {
                method: "redirect_uris_take_one",
                value: "There are one or more elements present.".to_string(),
            });
        }
        Ok(Vec::from(self).remove(0))
    }
}

impl IntoIterator for RedirectUris {
    type Item = RedirectUri;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<RedirectUri> for RedirectUris {
    fn from_iter<T: IntoIterator<Item = RedirectUri>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect::<Vec<_>>())
    }
}

impl From<RedirectUris> for Vec<RedirectUri> {
    fn from(value: RedirectUris) -> Self {
        value.0.into_iter().collect()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct RedirectUri(String);

impl RedirectUri {
    pub fn new(uri: impl Into<String>) -> Self {
        Self(uri.into())
    }
}

impl PartialEq<str> for RedirectUri {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
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
