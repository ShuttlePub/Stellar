use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
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