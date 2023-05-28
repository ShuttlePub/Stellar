use serde::{Deserialize, Serialize};
use crate::services::RandomizeService;



#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl PartialEq<str> for SessionId {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

impl From<SessionId> for String {
    fn from(value: SessionId) -> Self {
        value.0
    }
}

impl AsRef<str> for SessionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        RandomizeService::gen_str(128, Self::new)
    }
}