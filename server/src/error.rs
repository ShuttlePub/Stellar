use driver::DriverError;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    Driver(#[from] DriverError),
}