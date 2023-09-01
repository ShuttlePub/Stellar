use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Copy, Clone, Hash, Deserialize, Serialize)]
pub struct EstablishedAt(OffsetDateTime);

impl EstablishedAt {
    pub fn new(at: impl Into<OffsetDateTime>) -> Self {
        Self(at.into())
    }
}

impl From<EstablishedAt> for OffsetDateTime {
    fn from(value: EstablishedAt) -> Self {
        value.0
    }
}

impl AsRef<OffsetDateTime> for EstablishedAt {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl Default for EstablishedAt {
    fn default() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}
