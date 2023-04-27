use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct PolicyUri(String);

impl PolicyUri {
    pub fn new(uri: impl Into<String>) -> Self {
        Self(uri.into())
    }
}

impl From<PolicyUri> for String {
    fn from(origin: PolicyUri) -> Self {
        origin.0
    }
}

impl AsRef<str> for PolicyUri {
    fn as_ref(&self) -> &str {
        &self.0
    }
}