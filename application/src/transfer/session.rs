use kernel::prelude::entities::{Session, DestructSession};
use kernel::external::{OffsetDateTime, Uuid};

#[derive(Debug)]
pub struct SessionDto {
    pub id: String,
    pub usr: Uuid,
    pub exp: OffsetDateTime,
    pub est: OffsetDateTime
}

impl From<Session> for SessionDto {
    fn from(value: Session) -> Self {
        let DestructSession {
            id,
            usr,
            exp,
            est
        } = value.into_destruct();
        Self {
            id: id.into(),
            usr: usr.into(),
            exp: exp.into(),
            est: est.into()
        }
    }
}