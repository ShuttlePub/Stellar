mod est;
mod session_id;

pub use self::{est::*, session_id::*};
use destructure::Destructure;

use crate::entities::{ExpiredIn, UserId};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Hash, Deserialize, Serialize, Destructure)]
pub struct Session {
    id: SessionId,
    usr: UserId,
    exp: ExpiredIn,
    est: EstablishedAt,
}

impl Session {
    pub fn new(
        id: impl Into<String>,
        usr: impl Into<Uuid>,
        exp: impl Into<Duration>,
        est: impl Into<OffsetDateTime>,
    ) -> Self {
        Self {
            id: SessionId::new(id),
            usr: UserId::new(usr),
            exp: ExpiredIn::new(exp),
            est: EstablishedAt::new(est),
        }
    }
}

impl Session {
    pub fn id(&self) -> &SessionId {
        &self.id
    }

    pub fn usr(&self) -> &UserId {
        &self.usr
    }

    pub fn exp(&self) -> &ExpiredIn {
        &self.exp
    }

    pub fn est(&self) -> &EstablishedAt {
        &self.est
    }
}
