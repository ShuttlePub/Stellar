use kernel::external::Uuid;
use crate::{
    ApplicationError, 
    transfer::client::{
        ClientDto,
        RegisterClientDto
    }
};
use crate::transfer::client::UpdateClientDto;

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RegisterClientAdapter: 'static + Sync + Send {
    async fn register(&self, register: RegisterClientDto) -> Result<ClientDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait UpdateClientAdapter: 'static + Sync + Send {
    async fn update(&self, id: &Uuid, cl_secret: &str, pass_phrase: &str, update: UpdateClientDto) -> Result<ClientDto, ApplicationError>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait DeleteClientAdapter: 'static + Sync + Send {
    async fn delete(&self, id: &Uuid, cl_secret: &str, pass_phrase: &str) -> Result<(), ApplicationError>;
}