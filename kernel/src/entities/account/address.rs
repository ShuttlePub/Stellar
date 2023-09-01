use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Address(String);

impl Address {
    pub fn new(address: impl Into<String>) -> Self {
        Self(address.into())
    }
}

impl From<Address> for String {
    fn from(origin: Address) -> Self {
        origin.0
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
