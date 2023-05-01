use crate::{
    KernelError, 
    entities::{AccessToken, AccessTokenId, AuthorizeToken, AuthorizeTokenId}
};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait AuthorizeTokenRepository: 'static + Sync + Send {
    async fn create(&self, create: &AuthorizeToken) -> Result<(), KernelError>;
    async fn delete(&self, delete: &AuthorizeTokenId) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &AuthorizeTokenId) -> Result<Option<AuthorizeToken>, KernelError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait AccessTokenRepository: 'static + Sync + Send {
    async fn create(&self, create: &AccessToken) -> Result<(), KernelError>;
    async fn update(&self, update: &AccessToken) -> Result<(), KernelError>;
    async fn delete(&self, delete: &AccessTokenId) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &AccessTokenId) -> Result<Option<AccessToken>, KernelError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RefreshTokenRepository: 'static + Sync + Send {
    async fn create(&self) -> Result<(), KernelError>;
    async fn delete(&self) -> Result<(), KernelError>;

    async fn find(&self) -> Result<(), KernelError>;
}