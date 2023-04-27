use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct ClientUri(String);

impl ClientUri {
    pub fn new(uri: impl Into<String>) -> Self {
        Self(uri.into())
    }
}

impl From<ClientUri> for String {
    fn from(value: ClientUri) -> Self {
        value.0
    }
}

impl AsRef<str> for ClientUri {
    fn as_ref(&self) -> &str {
        &self.0
    }
}