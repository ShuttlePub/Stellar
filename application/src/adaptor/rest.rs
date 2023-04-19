use uuid::Uuid;

use crate::{
    ApplicationError,
    transfer::account::{
        CreateNonVerifiedAccountDto,
        NonVerifiedAccountDto, 
        CreateAccountDto, 
        AccountDto, 
        UpdateAccountDto
    }, 
};

#[async_trait::async_trait]
pub trait RestAdaptor: 'static + Send + Sync {
    async fn prepare_user_verification(&self, user: CreateNonVerifiedAccountDto) -> Result<NonVerifiedAccountDto, ApplicationError>;
    async fn approve_account(&self, ticket: &str, code: &str) -> Result<String, ApplicationError>;
    async fn create_account(&self, ticket: &str, create: CreateAccountDto) -> Result<AccountDto, ApplicationError>;
    async fn update_account(&self, update: UpdateAccountDto) -> Result<AccountDto, ApplicationError>;
    async fn delete_account(&self, pass: &str, delete: &Uuid) -> Result<(), ApplicationError>;
}