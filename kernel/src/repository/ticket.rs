use crate::entities::{TicketId, UserId};
use crate::KernelError;

#[async_trait::async_trait]
pub trait PendingActionVolatileRepository: 'static + Sync + Send {
    async fn create(&self, ticket: &TicketId, user_id: &UserId) -> Result<(), KernelError>;
    async fn revoke(&self, ticket: &TicketId) -> Result<(), KernelError>;

    async fn find(&self, ticket: &TicketId) -> Result<Option<UserId>, KernelError>;
}

pub trait DependOnPendingActionVolatileRepository: 'static + Sync + Send {
    type PendingActionVolatileRepository: PendingActionVolatileRepository;
    fn pending_action_volatile_repository(&self) -> &Self::PendingActionVolatileRepository;
}


#[async_trait::async_trait]
pub trait AcceptedActionVolatileRepository: 'static + Sync + Send {
    async fn create(&self, ticket: &TicketId, user_id: &UserId) -> Result<(), KernelError>;
    async fn revoke(&self, ticket: &TicketId) -> Result<(), KernelError>;

    async fn find(&self, ticket: &TicketId) -> Result<Option<UserId>, KernelError>;
}

pub trait DependOnAcceptedActionVolatileRepository: 'static + Sync + Send {
    type AcceptedActionVolatileRepository: AcceptedActionVolatileRepository;
    fn accepted_action_volatile_repository(&self) -> &Self::AcceptedActionVolatileRepository;
}