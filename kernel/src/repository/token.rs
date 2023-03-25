use crate::{
    KernelError, 
    entities::{Token, TokenId}
};

#[async_trait::async_trait]
pub trait TokenRepository: 'static + Sync + Send {
    async fn create(&self, create: &Token) -> Result<(), KernelError>;
    async fn update(&self, update: &Token) -> Result<(), KernelError>;
    async fn delete(&self, delete: &TokenId) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &TokenId) -> Result<Option<Token>, KernelError>;
}