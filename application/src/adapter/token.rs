use kernel::external::Uuid;

use crate::{transfer::token::{AuthorizeTokenDto, CreateAuthorizeTokenDto, CreateAccessTokenDto, AccessTokenDto}, ApplicationError};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait CreateAuthorizeTokenAdapter: 'static + Sync + Send {
    async fn create(&self, user: &Uuid, create: CreateAuthorizeTokenDto) -> Result<AuthorizeTokenDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait CreateAuthorizeTokenImplicitFlowAdapter: 'static + Sync + Send {
    async fn create(&self, user: &Uuid, create: CreateAuthorizeTokenDto) -> Result<AuthorizeTokenDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait DeleteAuthorizeTokenAdapter: 'static + Sync + Send {
    async fn delete(&self, id: &str) -> Result<(), ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait CreateAccessTokenAdapter: 'static + Sync + Send {
    async fn create(&self, create: CreateAccessTokenDto) -> Result<AccessTokenDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait UpdateAccessTokenAdapter: 'static + Sync + Send {
    async fn update(&self) -> Result<(), ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait DeleteAccessTokenAdapter: 'static + Sync + Send {
    async fn delete(&self, id: &str) -> Result<(), ApplicationError>;
}