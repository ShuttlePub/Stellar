use crate::KernelError;
use destructure::Destructure;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use super::time::{LoggedAt, VerifiedAt};

mod address;
mod pass;
mod user_id;
mod username;

pub use self::{address::*, pass::*, user_id::*, username::*};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, Destructure)]
pub struct Account {
    id: UserId,
    address: Address,
    name: UserName,
    pass: Password,
    date: LoggedAt,
    verified_at: VerifiedAt,
}

impl Account {
    pub fn new(
        id: impl Into<Uuid>,
        address: impl Into<String>,
        name: impl Into<String>,
        pass: impl Into<String>,
        created_at: impl Into<OffsetDateTime>,
        updated_at: impl Into<OffsetDateTime>,
        verified_at: impl Into<OffsetDateTime>,
    ) -> Result<Self, KernelError> {
        Ok(Self {
            id: UserId::new(id),
            address: Address::new(address),
            name: UserName::new(name),
            pass: Password::new(pass)?,
            date: LoggedAt::new(created_at, updated_at),
            verified_at: VerifiedAt::new(verified_at),
        })
    }

    pub fn new_with_unchecked(
        id: impl Into<Uuid>,
        address: impl Into<String>,
        name: impl Into<String>,
        pass: impl Into<String>,
        created_at: impl Into<OffsetDateTime>,
        updated_at: impl Into<OffsetDateTime>,
        verified_at: impl Into<OffsetDateTime>,
    ) -> Self {
        Self {
            id: UserId::new(id),
            address: Address::new(address),
            name: UserName::new(name),
            pass: Password::new_unchecked(pass),
            date: LoggedAt::new(created_at, updated_at),
            verified_at: VerifiedAt::new(verified_at),
        }
    }
}

impl Account {
    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn pass(&self) -> &Password {
        &self.pass
    }

    pub fn date(&self) -> &LoggedAt {
        &self.date
    }

    pub fn verified_at(&self) -> &VerifiedAt {
        &self.verified_at
    }
}

#[derive(Debug, Hash, Serialize, Deserialize, Destructure)]
pub struct TemporaryAccount {
    id: UserId,
    address: Address,
}

impl TemporaryAccount {
    pub fn new(id: impl Into<Uuid>, address: impl Into<String>) -> Self {
        Self {
            id: UserId::new(id),
            address: Address::new(address),
        }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}
