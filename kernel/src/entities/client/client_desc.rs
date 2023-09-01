use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct ClientDescription(String);

impl ClientDescription {
    pub fn new(description: impl Into<String>) -> Self {
        Self(description.into())
    }
}

impl From<ClientDescription> for String {
    fn from(origin: ClientDescription) -> Self {
        origin.0
    }
}

impl AsRef<str> for ClientDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
