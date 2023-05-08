use kernel::external::Uuid;
use crate::{
    ApplicationError,
    adapter::{
        rest::RestAdapter,
        account::{
            CreateNonVerifiedAccountAdapter,
            CreateAccountAdapter,
            UpdateAccountAdapter,
            DeleteAccountAdapter,
            ApproveAccountAdapter
        }
    },
    transfer::account::{
        CreateNonVerifiedAccountDto,
        NonVerifiedAccountDto, 
        CreateAccountDto, 
        AccountDto, 
        UpdateAccountDto
    },
};

#[derive(Clone)]
pub struct RestInteractor<T1, T2, T3, T4, T5> {
    nvac: T1,
    acv: T2,
    acc: T3,
    acu: T4,
    acd: T5
}

impl<T1, T2, T3, T4, T5> RestInteractor<T1, T2, T3, T4, T5> {
    pub fn new(nvac: T1, acv: T2, acc: T3, acu: T4, acd: T5) -> Self {
        Self { nvac, acv, acc, acu, acd }
    }
}

#[async_trait::async_trait]
impl<T1, T2, T3, T4, T5> RestAdapter for RestInteractor<T1, T2, T3, T4, T5>
  where T1: CreateNonVerifiedAccountAdapter,
        T2: ApproveAccountAdapter,
        T3: CreateAccountAdapter,
        T4: UpdateAccountAdapter,
        T5: DeleteAccountAdapter
{
    async fn prepare_user_verification(&self, user: CreateNonVerifiedAccountDto) -> Result<NonVerifiedAccountDto, ApplicationError> {
        self.nvac.create(user).await
    }

    async fn approve_account(&self, ticket: &str, code: &str) -> Result<String, ApplicationError> {
        self.acv.approve(ticket, code).await
    }

    async fn create_account(&self, ticket: &str, create: CreateAccountDto) -> Result<AccountDto, ApplicationError> {
        self.acc.create(ticket, create).await
    }

    async fn update_account(&self, update: UpdateAccountDto) -> Result<AccountDto, ApplicationError> {
        self.acu.update(update).await
    }
    
    async fn delete_account(&self, pass: &str, delete: &Uuid) -> Result<(), ApplicationError> {
        self.acd.delete(pass, delete).await
    }
}