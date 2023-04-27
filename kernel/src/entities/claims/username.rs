use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl From<UserName> for String {
    fn from(origin: UserName) -> Self {
        origin.0
    }
}

impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}