use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Issuer(String);

impl Issuer {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<Issuer> for String {
    fn from(origin: Issuer) -> Self {
        origin.0
    }
}

impl AsRef<str> for Issuer {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
