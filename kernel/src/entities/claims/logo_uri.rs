use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct LogoUri(String);

impl LogoUri {
    pub fn new(uri: impl Into<String>) -> Self {
        Self(uri.into())
    }
}

impl From<LogoUri> for String {
    fn from(value: LogoUri) -> Self {
        value.0
    }
}

impl AsRef<str> for LogoUri {
    fn as_ref(&self) -> &str {
        &self.0
    }
}