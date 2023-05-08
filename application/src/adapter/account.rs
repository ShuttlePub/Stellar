use kernel::external::Uuid;
use crate::{
    ApplicationError, 
    transfer::account::{
        CreateNonVerifiedAccountDto,
        NonVerifiedAccountDto, 
        CreateAccountDto, 
        UpdateAccountDto, 
        AccountDto, 
    }
};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait CreateNonVerifiedAccountAdapter: 'static + Send + Sync {
    async fn create(&self, create: CreateNonVerifiedAccountDto) -> Result<NonVerifiedAccountDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait ApproveAccountAdapter: 'static + Send + Sync {
    async fn approve(&self, id: &str, code: &str) -> Result<String, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait CreateAccountAdapter: 'static + Send + Sync {
    async fn create(&self, id: &str, create: CreateAccountDto) -> Result<AccountDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait UpdateAccountAdapter: 'static + Send + Sync {
    async fn update(&self, update: UpdateAccountDto) -> Result<AccountDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait DeleteAccountAdapter: 'static + Send + Sync {
    async fn delete(&self, pass: &str, delete: &Uuid) -> Result<(), ApplicationError>;
}