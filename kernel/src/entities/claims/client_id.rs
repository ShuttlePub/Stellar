use std::fmt::Display;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ClientId(Uuid);

impl ClientId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl From<ClientId> for Uuid {
    fn from(origin: ClientId) -> Self {
        origin.0
    }
}

impl AsRef<Uuid> for ClientId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}