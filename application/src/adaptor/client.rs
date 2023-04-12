use crate::ApplicationError;

#[async_trait::async_trait]
pub trait RegisterClientAdaptor: 'static + Sync + Send {
    async fn register(&self) -> Result<(), ApplicationError>;
}

#[async_trait::async_trait]
pub trait RegisterClientScopeAdaptor: 'static + Sync + Send {
    async fn register(&self) -> Result<(), ApplicationError>;
}

#[async_trait::async_trait]
pub trait UnRegisterClientAdaptor: 'static + Sync + Send {
    async fn unregister(&self) -> Result<(), ApplicationError>;
}

#[async_trait::async_trait]
pub trait UnRegisterClientScopeAdaptor: 'static + Sync + Send {
    async fn unregister(&self) -> Result<(), ApplicationError>;
}

#[async_trait::async_trait]
pub trait UpdateClientAdaptor: 'static + Sync + Send {
    async fn update(&self) -> Result<(), ApplicationError>;
}

#[async_trait::async_trait]
pub trait UpdateClientScopeAdaptor: 'static + Sync + Send {
    async fn update(&self) -> Result<(), ApplicationError>;
}