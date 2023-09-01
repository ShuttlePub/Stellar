use destructure::Destructure;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize, Destructure)]
pub struct ClientId {
    id: Uuid,
    issued_at: OffsetDateTime,
}

impl ClientId {
    pub fn new(id: impl Into<Uuid>, iat: impl Into<OffsetDateTime>) -> Self {
        Self {
            id: id.into(),
            issued_at: iat.into(),
        }
    }

    pub fn new_at_now(id: impl Into<Uuid>) -> Self {
        Self {
            id: id.into(),
            issued_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn issued_at(&self) -> &OffsetDateTime {
        &self.issued_at
    }
}

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, iat: {}", self.id, self.issued_at)
    }
}

impl Default for ClientId {
    fn default() -> Self {
        Self::new_at_now(Uuid::new_v4())
    }
}

impl From<ClientId> for Uuid {
    fn from(value: ClientId) -> Self {
        value.id
    }
}
