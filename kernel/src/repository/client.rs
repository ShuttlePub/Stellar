use crate::{entities::{Client, ClientId}, KernelError};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait ClientRegistry: 'static + Sync + Send {
    async fn register(&self, client: &Client) -> Result<(), KernelError>;
    async fn delete(&self, id: &ClientId) -> Result<(), KernelError>;
    async fn update(&self, client: &Client) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &ClientId) -> Result<Option<Client>, KernelError>;
}