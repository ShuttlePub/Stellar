use crate::KernelError;
use std::collections::HashSet;

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait BlackListTransporter: 'static + Sync + Send {
    async fn pull(&self) -> Result<HashSet<String>, KernelError>;
}

pub trait DependOnBlacklistTransporter: 'static + Sync + Send {
    type BlacklistTransporter: BlackListTransporter;
    fn blacklist_transporter(&self) -> Self::BlacklistTransporter;
}
