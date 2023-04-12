use serde::{Deserialize, Serialize};

use crate::KernelError;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ClientSecret(String);

impl ClientSecret {
    pub fn new(secret: impl Into<String>) -> Self {
        Self(secret.into())
    }

    pub fn verify(&self, _secret: impl Into<String>) -> Result<(), KernelError> {
        todo!()
    }
}

impl From<ClientSecret> for String {
    fn from(origin: ClientSecret) -> Self {
        origin.0
    }
}

impl AsRef<str> for ClientSecret {
    fn as_ref(&self) -> &str {
        &self.0
    }
}