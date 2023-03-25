use kernel::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error(transparent)]
    SqlX(#[from] sqlx::Error),
    #[error(transparent)]
    DeadPool(anyhow::Error),
    #[error(transparent)]
    Redis(#[from] deadpool_redis::redis::RedisError),
    #[error(transparent)]
    Lettre(anyhow::Error),
    #[error(transparent)]
    Kernel(#[from] KernelError)
}

impl From<DriverError> for KernelError {
    fn from(origin: DriverError) -> Self {
        match origin {
            DriverError::SqlX(e) => KernelError::Driver(anyhow::Error::new(e)),
            DriverError::DeadPool(e) => KernelError::Driver(e),
            DriverError::Redis(e) => KernelError::Driver(anyhow::Error::new(e)),
            DriverError::Lettre(e) => KernelError::Driver(e),
            DriverError::Kernel(internal) => internal,
        }
    }
}

impl From<deadpool_redis::PoolError> for DriverError {
    fn from(e: deadpool_redis::PoolError) -> Self {
        Self::DeadPool(anyhow::Error::new(e))
    }
}

impl From<deadpool_redis::CreatePoolError> for DriverError {
    fn from(e: deadpool_redis::CreatePoolError) -> Self {
        Self::DeadPool(anyhow::Error::new(e))
    }
}

impl From<lettre::transport::smtp::Error> for DriverError {
    fn from(e: lettre::transport::smtp::Error) -> Self {
        Self::Lettre(anyhow::Error::new(e))
    }
}

impl From<lettre::address::AddressError> for DriverError {
    fn from(e: lettre::address::AddressError) -> Self {
        Self::Lettre(anyhow::Error::new(e))
    }
}

impl From<lettre::error::Error> for DriverError {
    fn from(e: lettre::error::Error) -> Self {
        Self::Lettre(anyhow::Error::new(e))
    }
}