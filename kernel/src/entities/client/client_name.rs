use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ClientName(String);

impl ClientName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl From<ClientName> for String {
    fn from(origin: ClientName) -> Self {
        origin.0
    }
}

impl AsRef<str> for ClientName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}