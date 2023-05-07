use std::collections::HashSet;
use crate::KernelError;

#[async_trait::async_trait]
pub trait BlackListTransporter: 'static + Sync + Send {
   async fn pull(&self) -> Result<HashSet<String>, KernelError>;
}