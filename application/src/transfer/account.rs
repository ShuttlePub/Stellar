use kernel::entities::{Account, DestructAccount, DestructLoggedAt, NonVerifiedAccount, DestructNonVerifiedAccount};
use kernel::external::{OffsetDateTime, Uuid};

#[derive(Debug)]
pub struct AccountDto {
    pub id: Uuid,
    pub address: String,
    pub name: String,
    pub pass: String,
    pub updated_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub verified_at: OffsetDateTime
}

impl From<Account> for AccountDto {
    fn from(origin: Account) -> Self {
        let DestructAccount {
            id,
            address,
            name,
            pass,
            date,
            verified_at,
        } = origin.into_destruct();
        let DestructLoggedAt {
            created_at,
            updated_at,
        } = date.into_destruct();
        Self { 
            id: id.into(), 
            address: address.into(), 
            name: name.into(), 
            pass: pass.into(), 
            updated_at: updated_at.into(), 
            created_at: created_at.into(), 
            verified_at: verified_at.into()
        }
    }
}

#[derive(Debug)]
pub struct CreateAccountDto {
    pub name: String,
    pub pass: String
}

impl CreateAccountDto {
    pub fn new(
        name: impl Into<String>,
        pass: impl Into<String>
    ) -> Self {
        Self { 
            name: name.into(), 
            pass: pass.into()
        }
    }
}

#[derive(Debug)]
pub struct UpdateAccountDto {
    pub id: Uuid,
    pub address: String,
    pub name: String,
    pub pass: String
}

impl UpdateAccountDto {
    pub fn new(
        id: impl Into<Uuid>,
        address: impl Into<String>,
        name: impl Into<String>,
        pass: impl Into<String>
    ) -> Self {
        Self { 
            id: id.into(),
            address: address.into(),
            name: name.into(), 
            pass: pass.into()
        }
    }
}


#[derive(Debug)]
pub struct VerifyAccountDto {
    pub address: Option<String>,
    pub pass: Option<String>,
    pub ticket: Option<String>,
    pub session: Option<String>,
}


#[derive(Debug)]
pub struct NonVerifiedAccountDto {
    pub id: String,
    pub address: String,
    pub code: String
}

impl NonVerifiedAccountDto {
    pub fn new(id: impl Into<String>, address: impl Into<String>, code: impl Into<String>) -> Self {
        Self { id: id.into(), address: address.into(), code: code.into() }
    }
}

impl From<NonVerifiedAccount> for NonVerifiedAccountDto {
    fn from(origin: NonVerifiedAccount) -> Self {
        let DestructNonVerifiedAccount {
            id,
            address,
            code,
        } = origin.into_destruct();
        Self { 
            id: id.into(),
            address: address.into(),
            code: code.into()
        }
    }
}

#[derive(Debug)]
pub struct CreateNonVerifiedAccountDto {
    pub address: String,
}

impl CreateNonVerifiedAccountDto {
    pub fn new(
        address: impl Into<String>,
    ) -> Self {
        Self { address: address.into() }
    }
}