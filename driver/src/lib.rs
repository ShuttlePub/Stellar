pub mod database;
pub mod transport;
mod error;
mod driver;

pub use self::error::*;
pub use self::driver::*;

pub(crate) type SmtpPool = lettre::AsyncSmtpTransport<lettre::Tokio1Executor>;