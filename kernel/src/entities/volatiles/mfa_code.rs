use serde::{Deserialize, Serialize};
use crate::services::RandomizeService;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct VerificationCode(String);

impl VerificationCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl From<VerificationCode> for String {
    fn from(origin: VerificationCode) -> Self {
        origin.0
    }
}

impl AsRef<str> for VerificationCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for VerificationCode {
    fn default() -> Self {
        RandomizeService::gen_str(8, VerificationCode::new)
    }
}