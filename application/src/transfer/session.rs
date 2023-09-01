use kernel::external::{OffsetDateTime, Uuid};
use kernel::prelude::entities::{DestructSession, Session};

#[derive(Debug)]
pub struct SessionDto {
    pub id: String,
    pub usr: Uuid,
    pub exp: OffsetDateTime,
    pub est: OffsetDateTime,
}

impl From<Session> for SessionDto {
    fn from(value: Session) -> Self {
        let DestructSession { id, usr, exp, est } = value.into_destruct();
        Self {
            id: id.into(),
            usr: usr.into(),
            exp: exp.into(),
            est: est.into(),
        }
    }
}
