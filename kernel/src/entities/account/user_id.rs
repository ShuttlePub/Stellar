use crate::KernelError;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl TryFrom<Vec<u8>> for UserId {
    type Error = KernelError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(Uuid::from_slice(value.as_slice())?))
    }
}

impl TryFrom<String> for UserId {
    type Error = KernelError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Uuid::from_str(value.as_str())?))
    }
}

impl From<UserId> for Uuid {
    fn from(origin: UserId) -> Self {
        origin.0
    }
}

impl AsRef<[u8]> for UserId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
