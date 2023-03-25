use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct NotBefore(OffsetDateTime);

impl NotBefore {
    fn new() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}

impl From<NotBefore> for OffsetDateTime {
    fn from(origin: NotBefore) -> Self {
        origin.0
    }
}

impl AsRef<OffsetDateTime> for NotBefore {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl Default for NotBefore {
    fn default() -> Self {
        Self::new()
    }
}