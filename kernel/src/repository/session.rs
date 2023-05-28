use crate::entities::SessionId;
use crate::KernelError;

#[async_trait::async_trait]
pub trait SessionVolatileRepository: 'static + Sync + Send {
    async fn establish(&self, token: &SessionId) -> Result<(), KernelError>;
    async fn revoke(&self, token: &SessionId) -> Result<(), KernelError>;
    async fn find(&self) -> Result<Option<()>, KernelError>;
}