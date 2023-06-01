use crate::{entities::{MFACode, Address}, KernelError};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait VerificationMailTransporter: 'static + Sync + Send {
    async fn send(&self, address: &Address, code: &MFACode) -> Result<(), KernelError>;
}

pub trait DependOnVerificationMailTransporter: 'static + Sync + Send {
    type VerificationMailTransporter: VerificationMailTransporter;
    fn verification_mail_transporter(&self) -> &Self::VerificationMailTransporter;
}