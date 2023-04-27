use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct VerifiedAt(OffsetDateTime);

impl VerifiedAt {
    pub fn new(verified: impl Into<OffsetDateTime>) -> Self {
        Self(verified.into())
    }
}

impl From<VerifiedAt> for OffsetDateTime {
    fn from(origin: VerifiedAt) -> Self {
        origin.0
    }
}

impl AsRef<OffsetDateTime> for VerifiedAt {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}