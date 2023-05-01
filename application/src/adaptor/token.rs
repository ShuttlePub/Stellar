use kernel::external::Uuid;

use crate::{transfer::token::{AuthorizeTokenDto, CreateAuthorizeTokenDto, CreateAccessTokenDto, AccessTokenDto}, ApplicationError};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait CreateAuthorizeTokenAdaptor: 'static + Sync + Send {
    async fn create(&self, user: &Uuid, create: CreateAuthorizeTokenDto) -> Result<AuthorizeTokenDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait CreateAuthorizeTokenImplicitFlowAdaptor: 'static + Sync + Send {
    async fn create(&self, user: &Uuid, create: CreateAuthorizeTokenDto) -> Result<AuthorizeTokenDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait DeleteAuthorizeTokenAdaptor: 'static + Sync + Send {
    async fn delete(&self, id: &str) -> Result<(), ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait CreateAccessTokenAdaptor: 'static + Sync + Send {
    async fn create(&self, create: CreateAccessTokenDto) -> Result<AccessTokenDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait UpdateAccessTokenAdaptor: 'static + Sync + Send {
    async fn update(&self) -> Result<(), ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait DeleteAccessTokenAdaptor: 'static + Sync + Send {
    async fn delete(&self, id: &str) -> Result<(), ApplicationError>;
}