use crate::{
    entities::{Account, UserId, NonVerifiedAccount, TicketId, Address}, 
    KernelError
};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait AccountRepository: 'static + Sync + Send {
    async fn create(&self, create: &Account) -> Result<(), KernelError>;
    async fn update(&self, update: &Account) -> Result<(), KernelError>;
    async fn delete(&self, delete: &UserId) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &UserId) -> Result<Option<Account>, KernelError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait NonVerifiedAccountRepository: 'static + Sync + Send {
    async fn create(&self, create: &NonVerifiedAccount) -> Result<(), KernelError>;
    async fn validation(&self, coupon: &TicketId, valid: &TicketId, address: &Address) -> Result<(), KernelError>;
    async fn find_by_id(&self, id: &TicketId) -> Result<Option<NonVerifiedAccount>, KernelError>;
    async fn find_by_valid_id(&self, id: &TicketId) -> Result<Option<Address>, KernelError>;
}