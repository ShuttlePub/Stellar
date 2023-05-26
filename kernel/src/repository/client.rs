use crate::{entities::{Client, ClientId}, KernelError};
use crate::entities::ClientName;

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait ClientRegistry: 'static + Sync + Send {
    async fn register(&self, client: &Client) -> Result<(), KernelError>;
    async fn delete(&self, id: &ClientId) -> Result<(), KernelError>;
    async fn update(&self, client: &Client) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &ClientId) -> Result<Option<Client>, KernelError>;
    async fn find_by_name(&self, name: &ClientName) -> Result<Option<Client>, KernelError>;
}

pub trait DependOnClientRegistry: 'static + Sync + Send {
    type ClientRegistry: ClientRegistry;
    fn client_registry(&self) -> &Self::ClientRegistry;
}