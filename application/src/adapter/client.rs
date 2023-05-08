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
pub trait RegisterClientAdapter: 'static + Sync + Send {
    async fn register(&self, register: RegisterClientDto) -> Result<ClientDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait UpdateClientAdapter: 'static + Sync + Send {
    async fn update(&self, client_id: &Uuid, client_secret: &str) -> Result<ClientDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait DeleteClientAdapter: 'static + Sync + Send {
    async fn delete(&self) -> Result<(), ApplicationError>;
}