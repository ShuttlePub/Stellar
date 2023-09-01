use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct IssuedAt(OffsetDateTime);

impl IssuedAt {
    fn new() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}

impl From<IssuedAt> for OffsetDateTime {
    fn from(origin: IssuedAt) -> Self {
        origin.0
    }
}

impl AsRef<OffsetDateTime> for IssuedAt {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl Default for IssuedAt {
    fn default() -> Self {
        Self::new()
    }
}
