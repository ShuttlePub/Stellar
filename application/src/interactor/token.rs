use kernel::repository::{DependOnAccountRepository, DependOnAuthorizeTokenRepository, DependOnClientRegistry, DependOnPKCEVolatileRepository, DependOnStateVolatileRepository};
use crate::services::{AcceptAuthorizeTokenService, PendingAuthorizeTokenService, RejectAuthorizeTokenService};

impl<T> PendingAuthorizeTokenService for T
    where T: DependOnClientRegistry
           + DependOnPKCEVolatileRepository
           + DependOnAuthorizeTokenRepository
           + DependOnStateVolatileRepository {}

impl<T> AcceptAuthorizeTokenService for T
    where T: DependOnAccountRepository
           + DependOnStateVolatileRepository
           + DependOnAuthorizeTokenRepository {}

impl<T> RejectAuthorizeTokenService for T
    where T: DependOnAuthorizeTokenRepository {}