use crate::entities::{Session, SessionId};
use crate::KernelError;

#[async_trait::async_trait]
pub trait SessionVolatileRepository: 'static + Sync + Send {
    async fn establish(&self, session: &Session) -> Result<(), KernelError>;
    async fn revoke(&self, id: &SessionId) -> Result<(), KernelError>;
    async fn find(&self, id: &SessionId) -> Result<Option<Session>, KernelError>;
}

pub trait DependOnSessionVolatileRepository: 'static + Sync + Send {
    type SessionVolatileRepository: SessionVolatileRepository;
    fn session_volatile_repository(&self) -> &Self::SessionVolatileRepository;
}