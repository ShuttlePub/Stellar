mod entities;
mod error;
mod repository;
mod services;
mod transport;

use once_cell::sync::Lazy;
use url::Url;

pub use self::error::KernelError;

#[cfg(feature = "prelude")]
pub mod prelude {
    pub mod entities {
        pub use crate::entities::*;
    }
    pub mod services {
        pub use crate::services::*;
    }
}

#[cfg(feature = "interfaces")]
pub mod interfaces {
    pub mod repository {
        pub use crate::repository::*;
    }
    pub mod transport {
        pub use crate::transport::*;
    }
}

pub(crate) static BASE_URL: Lazy<Url> = Lazy::new(|| {
    dotenvy::var("BASE_URL")
        .ok()
        .map(|url| Url::parse(&url))
        .transpose()
        .expect("`BASE_URL` cannot parse url! This value require valid.")
        .expect("`BASE_URL` does not set! This value require.")
});

#[cfg(feature = "url")]
#[cfg(feature = "time")]
#[cfg(feature = "uuid")]
#[cfg(feature = "jsonwebkey")]
#[allow(ambiguous_glob_reexports)]
pub mod external {
    #[cfg(feature = "jsonwebkey")]
    pub use jsonwebkey::Error as JWKError;
    #[cfg(feature = "jsonwebkey")]
    pub use jsonwebkey::*;
    #[cfg(feature = "time")]
    pub use time::Error as TimeError;
    #[cfg(feature = "time")]
    pub use time::*;
    #[cfg(feature = "url")]
    pub use url::ParseError as UrlParseError;
    #[cfg(feature = "url")]
    pub use url::*;
    #[cfg(feature = "uuid")]
    pub use uuid::Error as UuidError;
    #[cfg(feature = "uuid")]
    pub use uuid::*;
}
