use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdatedAt(OffsetDateTime);

impl UpdatedAt {
    pub fn new(time: OffsetDateTime) -> Self {
        Self(time)
    }
}

impl From<UpdatedAt> for OffsetDateTime {
    fn from(date: UpdatedAt) -> Self {
        date.0
    }
}

impl AsRef<OffsetDateTime> for UpdatedAt {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}
