use rand::{distributions::Alphanumeric, prelude::Distribution};
use serde::{Serialize, Deserialize};
use destructure::Destructure;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::KernelError;

use super::{UserId, Address, Password, UpdateTime, UserName, VerifiedAt};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, Destructure)]
pub struct Account {
    id: UserId,
    address: Address,
    name: UserName,
    pass: Password,
    date: UpdateTime,
    verified_at: VerifiedAt
}

impl Account {
    pub fn new(
        id: impl Into<Uuid>,
        address: impl Into<String>,
        name: impl Into<String>,
        pass: impl Into<String>,
        created_at: impl Into<OffsetDateTime>,
        updated_at: impl Into<OffsetDateTime>,
        verified_at: impl Into<OffsetDateTime>
    ) -> Result<Self, KernelError> {
        Ok(Self { 
            id: UserId::new(id),
            address: Address::new(address),
            name: UserName::new(name),
            pass: Password::new(pass)?,
            date: UpdateTime::new(created_at, updated_at),
            verified_at: VerifiedAt::new(verified_at)
        })
    }

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

    pub fn date(&self) -> &UpdateTime {
        &self.date
    }

    pub fn verified_at(&self) -> &VerifiedAt {
        &self.verified_at
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketId(String);

impl TicketId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<TicketId> for String {
    fn from(origin: TicketId) -> Self {
        origin.0
    }
}

impl AsRef<str> for TicketId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for TicketId {
    fn default() -> Self {
        let ticket = Alphanumeric.sample_iter(&mut rand::thread_rng())
            .take(64)
            .map(char::from)
            .collect::<String>();
        Self::new(ticket)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct VerificationCode(String);

impl VerificationCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl From<VerificationCode> for String {
    fn from(origin: VerificationCode) -> Self {
        origin.0
    }
}

impl AsRef<str> for VerificationCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for VerificationCode {
    fn default() -> Self {
        let code = Alphanumeric.sample_iter(&mut rand::thread_rng())
            .take(8)
            .map(char::from)
            .collect::<String>();
        Self(code)
    }
}

#[derive(Debug, Hash, Serialize, Deserialize, Destructure)]
pub struct NonVerifiedAccount {
    id: TicketId,
    address: Address,
    code: VerificationCode,
}

impl NonVerifiedAccount {
    pub fn new(id: impl Into<String>, address: impl Into<String>, code: impl Into<String>) -> Self {
        Self { id: TicketId::new(id), address: Address::new(address), code: VerificationCode::new(code) }
    }

    pub fn is_match_verification_code(&self, code: &VerificationCode) -> bool {
        self.code.as_ref() == code.as_ref()
    }

    pub fn id(&self) -> &TicketId {
        &self.id
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn code(&self) -> &VerificationCode {
        &self.code
    }
}