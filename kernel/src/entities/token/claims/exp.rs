use serde::{Serialize, Deserialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ExpiredIn(OffsetDateTime);

impl ExpiredIn {
    pub fn new(expire_sec: impl Into<Duration>) -> Self {
        Self(OffsetDateTime::now_utc() + expire_sec.into())
    }

    pub fn is_expired(&self) -> bool {
        self.0 < OffsetDateTime::now_utc()
    }
}

impl From<ExpiredIn> for OffsetDateTime {
    fn from(origin: ExpiredIn) -> Self {
        origin.0
    }
}

impl AsRef<OffsetDateTime> for ExpiredIn {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}