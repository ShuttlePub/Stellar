use kernel::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error(transparent)]
    SqlX(#[from] sqlx::Error),
    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    DeadPool(anyhow::Error),
    #[error(transparent)]
    Redis(#[from] deadpool_redis::redis::RedisError),
    #[error(transparent)]
    Lettre(anyhow::Error),
    #[error(transparent)]
    Reqwest(anyhow::Error),
    #[error(transparent)]
    Serde(anyhow::Error),
    #[error(transparent)]
    FileSystem(anyhow::Error),
    #[error(transparent)]
    Kernel(#[from] KernelError),
}

impl From<DriverError> for KernelError {
    fn from(origin: DriverError) -> Self {
        match origin {
            DriverError::SqlX(e) => KernelError::Driver(anyhow::Error::new(e)),
            DriverError::Migration(e) => KernelError::Driver(anyhow::Error::new(e)),
            DriverError::Reqwest(e) => KernelError::Driver(e),
            DriverError::DeadPool(e) => KernelError::Driver(e),
            DriverError::Redis(e) => KernelError::Driver(anyhow::Error::new(e)),
            DriverError::Lettre(e) => KernelError::Driver(e),
            DriverError::Serde(e) => KernelError::Driver(e),
            DriverError::FileSystem(e) => KernelError::Driver(e),
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

impl From<reqwest::Error> for DriverError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(anyhow::Error::new(e))
    }
}

impl From<serde_json::Error> for DriverError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(anyhow::Error::new(e))
    }
}

impl From<toml::de::Error> for DriverError {
    fn from(e: toml::de::Error) -> Self {
        Self::Serde(anyhow::Error::new(e))
    }
}

impl From<toml::ser::Error> for DriverError {
    fn from(e: toml::ser::Error) -> Self {
        Self::Serde(anyhow::Error::new(e))
    }
}

impl From<rmp_serde::encode::Error> for DriverError {
    fn from(e: rmp_serde::encode::Error) -> Self {
        Self::Serde(anyhow::Error::new(e))
    }
}

impl From<rmp_serde::decode::Error> for DriverError {
    fn from(e: rmp_serde::decode::Error) -> Self {
        Self::Serde(anyhow::Error::new(e))
    }
}

impl From<std::io::Error> for DriverError {
    fn from(e: std::io::Error) -> Self {
        Self::FileSystem(anyhow::Error::new(e))
    }
}
