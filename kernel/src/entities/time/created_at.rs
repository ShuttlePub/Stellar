use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatedAt(OffsetDateTime);

impl CreatedAt {
    pub fn new(time: OffsetDateTime) -> Self {
        Self(time)
    }
}

impl From<CreatedAt> for OffsetDateTime {
    fn from(date: CreatedAt) -> Self {
        date.0
    }
}

impl AsRef<OffsetDateTime> for CreatedAt {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}