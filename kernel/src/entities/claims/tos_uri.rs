use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct TermsUri(String);

impl TermsUri {
    pub fn new(uri: impl Into<String>) -> Self {
        Self(uri.into())
    }
}

impl From<TermsUri> for String {
    fn from(value: TermsUri) -> Self {
        value.0
    }
}

impl AsRef<str> for TermsUri {
    fn as_ref(&self) -> &str {
        &self.0
    }
}