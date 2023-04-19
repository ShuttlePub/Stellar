use uuid::Uuid;

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

#[async_trait::async_trait]
pub trait CreateNonVerifiedAccountAdaptor: 'static + Send + Sync {
    async fn create(&self, create: CreateNonVerifiedAccountDto) -> Result<NonVerifiedAccountDto, ApplicationError>;
}

#[async_trait::async_trait]
pub trait ApproveAccountAdaptor: 'static + Send + Sync {
    async fn approve(&self, id: &str, code: &str) -> Result<String, ApplicationError>;
}

#[async_trait::async_trait]
pub trait CreateAccountAdaptor: 'static + Send + Sync {
    async fn create(&self, id: &str, create: CreateAccountDto) -> Result<AccountDto, ApplicationError>;
}

#[async_trait::async_trait]
pub trait UpdateAccountAdaptor: 'static + Send + Sync {
    async fn update(&self, update: UpdateAccountDto) -> Result<AccountDto, ApplicationError>;
}

#[async_trait::async_trait]
pub trait DeleteAccountAdaptor: 'static + Send + Sync {
    async fn delete(&self, pass: &str, delete: &Uuid) -> Result<(), ApplicationError>;
}