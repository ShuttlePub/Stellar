use crate::services::{
    AcceptAuthorizeTokenService, PendingAuthorizeTokenService, RejectAuthorizeTokenService,
};
use kernel::interfaces::repository::{
    DependOnAccountRepository, DependOnAuthorizeTokenRepository, DependOnClientRegistry,
    DependOnPKCEVolatileRepository, DependOnPendingAuthorizeTokenRepository,
    DependOnStateVolatileRepository,
};

impl<T> PendingAuthorizeTokenService for T where
    T: DependOnClientRegistry
        + DependOnPKCEVolatileRepository
        + DependOnPendingAuthorizeTokenRepository
        + DependOnStateVolatileRepository
{
}

impl<T> AcceptAuthorizeTokenService for T where
    T: DependOnAccountRepository
        + DependOnStateVolatileRepository
        + DependOnPendingAuthorizeTokenRepository
        + DependOnAuthorizeTokenRepository
{
}

impl<T> RejectAuthorizeTokenService for T where
    T: DependOnPendingAuthorizeTokenRepository
        + DependOnPKCEVolatileRepository
        + DependOnStateVolatileRepository
{
}
