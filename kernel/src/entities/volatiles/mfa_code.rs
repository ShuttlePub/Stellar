use crate::services::RandomizeService;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct MFACode(String);

impl MFACode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl From<MFACode> for String {
    fn from(origin: MFACode) -> Self {
        origin.0
    }
}

impl AsRef<str> for MFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for MFACode {
    fn default() -> Self {
        RandomizeService::gen_str(8, MFACode::new)
    }
}
