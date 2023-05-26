pub mod entities;
pub mod repository;
pub mod transport;
mod error;
mod services;

use once_cell::sync::Lazy;
use url::Url;
pub use self::error::KernelError;

pub(crate) static BASE_URL: Lazy<Url> = Lazy::new(|| {
    dotenvy::var("BASE_URL").ok()
        .map(|url| Url::parse(&url))
        .transpose()
        .expect("`BASE_URL` cannot parse url! This value require valid.")
        .expect("`BASE_URL` does not set! This value require.")
});

#[cfg(feature = "url")]
#[cfg(feature = "time")]
#[cfg(feature = "uuid")]
#[cfg(feature = "jsonwebkey")]
pub mod external {
    #[cfg(feature = "jsonwebkey")]
    pub use jsonwebkey::*;
    #[cfg(feature = "url")]
    pub use url::*;
    #[cfg(feature = "uuid")]
    pub use uuid::*;
    #[cfg(feature = "time")]
    pub use time::*;
}