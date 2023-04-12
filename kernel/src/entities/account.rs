use std::fmt::Display;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, prelude::Distribution, rngs::OsRng};
use serde::{Serialize, Deserialize};
use destructure::Destructure;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::KernelError;

use super::UpdateTime;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl From<UserId> for Uuid {
    fn from(origin: UserId) -> Self {
        origin.0
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Address(String);

impl Address {
    pub fn new(address: impl Into<String>) -> Self {
        Self(address.into())
    }
}

impl From<Address> for String {
    fn from(origin: Address) -> Self {
        origin.0
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl From<UserName> for String {
    fn from(origin: UserName) -> Self {
        origin.0
    }
}

impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

static ARGON: Lazy<Argon2> = Lazy::new(Argon2::default);

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Password(String);

impl Password {
    pub fn new(pass: impl Into<String>) -> Result<Self, KernelError> {
        let pass: String = pass.into();
        let salt = SaltString::generate(&mut OsRng);
        let pass  = ARGON.hash_password(pass.as_bytes(), &salt)
            .map_err(KernelError::Cryption)?
            .to_string();
        Ok(Self(pass))
    }

    pub fn unchecked_new(pass: impl Into<String>) -> Self {
        Self(pass.into())
    }

    /// Verifies passwords. 
    /// 
    /// If a match is found, `()` is returned; 
    /// if no match is found or an error occurs, [KernelError] is returned.
    /// 
    /// For an implementation, See [argon2::password_hash::PasswordVerifier] functions
    pub fn verify(&self, pass: impl Into<String>) -> Result<(), KernelError> {
        let self_hasing = PasswordHash::new(&self.0)
            .map_err(KernelError::Cryption)?;
        ARGON.verify_password(pass.into().as_bytes(), &self_hasing)
                .map_err(KernelError::Cryption)?;
        Ok(())
    }
}

impl From<Password> for String {
    fn from(origin: Password) -> Self {
        origin.0
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

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
            .take(32)
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