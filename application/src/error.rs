use kernel::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("cannot find `{id}:{entity}` in the following {method}.")]
    NotFound {
        method: &'static str,
        entity: &'static str,
        id: String
    },
    #[error("invalid value `{value}` in the following {method}.")]
    InvalidValue {
        method: &'static str,
        value: &'static str
    },
    #[error("failed verification `{id}:{entity}` in following {method}.")]
    Verification {
        method: &'static str,
        entity: &'static str,
        id: String,
    },
    #[error(transparent)]
    Other(anyhow::Error),
    #[error(transparent)]
    External(anyhow::Error)
}

impl From<KernelError> for ApplicationError {
    fn from(e: KernelError) -> Self {
        match e {
            KernelError::NotFound { method, entity, id } => ApplicationError::NotFound { method, entity, id },
            KernelError::InvalidValue { method, value } => ApplicationError::InvalidValue { method, value },
            KernelError::Driver(not_here) => ApplicationError::Other(not_here),
            KernelError::External(e) => ApplicationError::External(e),
            KernelError::InvalidPassword(e) => ApplicationError::Other(anyhow::Error::new(e)),
            KernelError::Cryption(e) => ApplicationError::Other(anyhow::Error::new(e))
        }
    }
}