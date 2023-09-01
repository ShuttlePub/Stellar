use crate::{
    entities::{Account, Address, TemporaryAccount, UserId},
    KernelError,
};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait AccountRepository: 'static + Sync + Send {
    async fn create(&self, create: &Account) -> Result<(), KernelError>;
    async fn update(&self, update: &Account) -> Result<(), KernelError>;
    async fn delete(&self, delete: &UserId) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &UserId) -> Result<Option<Account>, KernelError>;
    async fn find_by_address(&self, address: &Address) -> Result<Option<Account>, KernelError>;
}

pub trait DependOnAccountRepository: 'static + Sync + Send {
    type AccountRepository: AccountRepository;
    fn account_repository(&self) -> &Self::AccountRepository;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait TemporaryAccountRepository: 'static + Sync + Send {
    async fn create(&self, create: &TemporaryAccount) -> Result<(), KernelError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<TemporaryAccount>, KernelError>;
}

pub trait DependOnTemporaryAccountRepository: 'static + Sync + Send {
    type TemporaryAccountRepository: TemporaryAccountRepository;
    fn temporary_account_repository(&self) -> &Self::TemporaryAccountRepository;
}
