use kernel::repository::{DependOnAccountRepository, DependOnPendingAuthorizeTokenRepository, DependOnClientRegistry, DependOnPKCEVolatileRepository, DependOnStateVolatileRepository, DependOnAuthorizeTokenRepository};
use crate::services::{AcceptAuthorizeTokenService, PendingAuthorizeTokenService, RejectAuthorizeTokenService};

impl<T> PendingAuthorizeTokenService for T
    where T: DependOnClientRegistry
           + DependOnPKCEVolatileRepository
           + DependOnPendingAuthorizeTokenRepository
           + DependOnStateVolatileRepository {}

impl<T> AcceptAuthorizeTokenService for T
    where T: DependOnAccountRepository
           + DependOnStateVolatileRepository
           + DependOnPendingAuthorizeTokenRepository
           + DependOnAuthorizeTokenRepository {}

impl<T> RejectAuthorizeTokenService for T
    where T: DependOnPendingAuthorizeTokenRepository
           + DependOnPKCEVolatileRepository
           + DependOnStateVolatileRepository {}
