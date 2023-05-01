use rand::distributions::{Alphanumeric, Distribution};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct RegistrationAccessToken(String);

impl RegistrationAccessToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self(token.into())
    }
}

impl From<RegistrationAccessToken> for String {
    fn from(value: RegistrationAccessToken) -> Self {
        value.0
    }
}

impl AsRef<str> for RegistrationAccessToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for RegistrationAccessToken {
    fn default() -> Self {
        Self::new(
            Alphanumeric.sample_iter(&mut rand::thread_rng())
                .take(64)
                .map(char::from)
                .collect::<String>()
        )
    }
}