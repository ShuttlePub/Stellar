// Todo: impl to remove.
#![allow(unused)]

use kernel::{
    repository::{
        AuthorizeTokenRepository, 
        ClientRegistry, 
        AccountRepository
    }
};
use kernel::external::Uuid;

use crate::{
    ApplicationError,
    transfer::token::{
        CreateAuthorizeTokenDto,
        AuthorizeTokenDto
    },
    adaptor::token::CreateAuthorizeTokenAdaptor
};

#[derive(Clone)]
pub struct CreateAuthorizeTokenInteractor<T1, T2, T3> {
    kvs: T1,
    registry: T2,
    accounts: T3
}

impl<T1, T2, T3> CreateAuthorizeTokenInteractor<T1, T2, T3> {
    pub fn new(kvs: T1, registry: T2, accounts: T3) -> Self {
        Self { kvs, registry, accounts }
    }
}

#[async_trait::async_trait]
impl<T1, T2, T3> CreateAuthorizeTokenAdaptor for CreateAuthorizeTokenInteractor<T1, T2, T3>
  where T1: AuthorizeTokenRepository,
        T2: ClientRegistry,
        T3: AccountRepository
{
    async fn create(
        &self,
        user: &Uuid,
        create: CreateAuthorizeTokenDto
    ) -> Result<AuthorizeTokenDto, ApplicationError> {
        todo!()
    }
}