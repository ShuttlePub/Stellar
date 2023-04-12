use uuid::Uuid;

use crate::{transfer::token::{AuthorizeTokenDto, CreateAuthorizeTokenDto, CreateAccessTokenDto, AccessTokenDto}, ApplicationError};

#[async_trait::async_trait]
pub trait CreateAuthorizeTokenAdaptor: 'static + Sync + Send {
    async fn create(&self, user: &Uuid, create: CreateAuthorizeTokenDto) -> Result<AuthorizeTokenDto, ApplicationError>;
}

#[async_trait::async_trait]
pub trait DeleteAuthorizeTokenAdaptor: 'static + Sync + Send {
    async fn delete(&self, id: &str) -> Result<(), ApplicationError>;
}

#[async_trait::async_trait]
pub trait CreateAccessTokenAdaptor: 'static + Sync + Send {
    async fn create(&self, create: CreateAccessTokenDto) -> Result<AccessTokenDto, ApplicationError>;
}

#[async_trait::async_trait]
pub trait UpdateAccessTokenAdaptor: 'static + Sync + Send {
    async fn update(&self) -> Result<(), ApplicationError>;
}

#[async_trait::async_trait]
pub trait DeleteAccessTokenAdaptor: 'static + Sync + Send {
    async fn delete(&self, id: &str) -> Result<(), ApplicationError>;
}