use kernel::external::Uuid;
use crate::{
    ApplicationError, 
    transfer::client::{
        ClientDto,
        RegisterClientDto
    }
};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RegisterClientAdaptor: 'static + Sync + Send {
    async fn register(&self, register: RegisterClientDto) -> Result<ClientDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait UpdateClientAdaptor: 'static + Sync + Send {
    async fn update(&self, client_id: &Uuid, client_secret: &str) -> Result<ClientDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait DeleteClientAdaptor: 'static + Sync + Send {
    async fn delete(&self) -> Result<(), ApplicationError>;
}