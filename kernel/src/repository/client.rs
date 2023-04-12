use crate::{entities::{Client, ClientId}, KernelError};

#[async_trait::async_trait]
pub trait ClientRegistry: 'static + Sync + Send {
    async fn register(&self, client: &Client) -> Result<(), KernelError>;
    async fn unregister(&self, id: &ClientId) -> Result<(), KernelError>;

    async fn update(&self, client: &Client) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &ClientId) -> Result<Option<Client>, KernelError>;
}

#[async_trait::async_trait]
pub trait ClientScopeRegistry: 'static + Sync + Send {
    async fn register(&self) -> Result<(), KernelError>;
    async fn unregister(&self) -> Result<(), KernelError>;

    async fn update(&self) -> Result<(), KernelError>;

    async fn find_by_client_id(&self) -> Result<(), KernelError>;
    async fn find_by_scope(&self) -> Result<(), KernelError>;
}