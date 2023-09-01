use crate::services::RandomizeService;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketId(String);

impl TicketId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<TicketId> for String {
    fn from(origin: TicketId) -> Self {
        origin.0
    }
}

impl AsRef<str> for TicketId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for TicketId {
    fn default() -> Self {
        RandomizeService::gen_str(128, TicketId::new)
    }
}
