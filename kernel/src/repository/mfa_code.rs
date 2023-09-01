use crate::entities::{MFACode, UserId};
use crate::KernelError;

#[async_trait::async_trait]
pub trait MFACodeVolatileRepository: 'static + Sync + Send {
    async fn create(&self, user_id: &UserId, code: &MFACode) -> Result<(), KernelError>;
    async fn revoke(&self, user_id: &UserId) -> Result<(), KernelError>;

    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<MFACode>, KernelError>;
}

pub trait DependOnMFACodeVolatileRepository: 'static + Sync + Send {
    type MFACodeVolatileRepository: MFACodeVolatileRepository;
    fn mfa_code_volatile_repository(&self) -> &Self::MFACodeVolatileRepository;
}
