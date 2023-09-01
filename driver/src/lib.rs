mod config;
pub mod database;
mod driver;
mod error;
pub mod transport;

pub use self::driver::*;
pub use self::error::*;

pub(crate) type SmtpPool = lettre::AsyncSmtpTransport<lettre::Tokio1Executor>;
