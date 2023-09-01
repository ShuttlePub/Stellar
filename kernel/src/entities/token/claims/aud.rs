use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Audience(String);

impl Audience {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<Audience> for String {
    fn from(origin: Audience) -> Self {
        origin.0
    }
}

impl AsRef<str> for Audience {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
