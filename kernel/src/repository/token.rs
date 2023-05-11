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

pub trait DependOnAuthorizeTokenRepository: 'static + Sync + Send {
    type AuthorizeTokenRepository: AuthorizeTokenRepository;
    fn authorize_token_repository(&self) -> Self::AuthorizeTokenRepository;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait AccessTokenRepository: 'static + Sync + Send {
    async fn create(&self, create: &AccessToken) -> Result<(), KernelError>;
    async fn update(&self, update: &AccessToken) -> Result<(), KernelError>;
    async fn delete(&self, delete: &AccessTokenId) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &AccessTokenId) -> Result<Option<AccessToken>, KernelError>;
}

pub trait DependOnAccessTokenRepository: 'static + Sync + Send {
    type AccessTokenRepository: AccessTokenRepository;
    fn access_token_repository(&self) -> Self::AccessTokenRepository;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RefreshTokenRepository: 'static + Sync + Send {
    async fn create(&self) -> Result<(), KernelError>;
    async fn delete(&self) -> Result<(), KernelError>;

    async fn find(&self) -> Result<(), KernelError>;
}

pub trait DependOnRefreshTokenRepository: 'static + Sync + Send {
    type RefreshTokenRepository: RefreshTokenRepository;
    fn refresh_token_repository(&self) -> Self::RefreshTokenRepository;
}