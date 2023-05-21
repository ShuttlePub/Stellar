use crate::{
    KernelError,
    entities::{
        AccessToken,
        AccessTokenId,
        AuthorizeToken,
        AuthorizeTokenId,
        CodeChallenge,
        State
    }
};
use crate::entities::TicketId;

/// Pending AuthorizeToken storage process is summarized.
///
/// Token is highly volatile data with a short survival time,
/// so I expect them to be implemented in an in-memory database such as Redis.
///
/// - The tokens represented by this Trait are tokens **awaiting approval that have no owner**.
/// - See [AuthorizeTokenRepository] for a Trait that handles approved tokens.
#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait PendingAuthorizeTokenRepository: 'static + Sync + Send {
    /// Store authorization token information.
    /// TicketId is used as a key to store.
    ///
    /// ---
    ///
    /// TicketId is used to simplify the data
    /// when a user accepts/rejects an authorization decision
    /// at the authorization decision endpoint.
    async fn save(&self, ticket: &TicketId, create: &AuthorizeToken) -> Result<(), KernelError>;

    /// Delete authorization Token information.
    /// Intended for use when a user Rejects an authorization decision.
    async fn dele(&self, ticket: &TicketId) -> Result<(), KernelError>;

    async fn find(&self, ticket: &TicketId) -> Result<Option<AuthorizeToken>, KernelError>;
}

pub trait DependOnPendingAuthorizeTokenRepository: 'static + Sync + Send {
    type PendingAuthorizeTokenRepository: PendingAuthorizeTokenRepository;
    fn pending_authorize_token_repository(&self) -> &Self::PendingAuthorizeTokenRepository;
}


#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait AuthorizeTokenRepository: 'static + Sync + Send {
    async fn save(&self, id: &AuthorizeTokenId, token: &AuthorizeToken) -> Result<(), KernelError>;
    async fn dele(&self, id: &AuthorizeTokenId) -> Result<(), KernelError>;
    async fn find(&self, id: &AuthorizeTokenId) -> Result<Option<AuthorizeToken>, KernelError>;
}

pub trait DependOnAuthorizeTokenRepository: 'static + Sync + Send {
    type AuthorizeTokenRepository: AuthorizeTokenRepository;
    fn authorize_token_repository(&self) -> &Self::AuthorizeTokenRepository;
}


#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait PKCEVolatileRepository: 'static + Sync + Send {
    async fn save(&self, token_id: &AuthorizeTokenId, code: &CodeChallenge) -> Result<(), KernelError>;
    async fn dele(&self, token_id: &AuthorizeTokenId) -> Result<(), KernelError>;
    async fn find(&self, token_id: &AuthorizeTokenId) -> Result<Option<CodeChallenge>, KernelError>;
}

pub trait DependOnPKCEVolatileRepository: 'static + Sync + Send {
    type PKCEVolatileRepository: PKCEVolatileRepository;
    fn pkce_volatile_repository(&self) -> &Self::PKCEVolatileRepository;
}


#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait StateVolatileRepository: 'static + Sync + Send {
    async fn save(&self, ticket: &TicketId, state: &State) -> Result<(), KernelError>;
    async fn dele(&self, ticket: &TicketId) -> Result<(), KernelError>;
    async fn find(&self, ticket: &TicketId) -> Result<Option<State>, KernelError>;
}

pub trait DependOnStateVolatileRepository: 'static + Sync + Send {
    type StateVolatileRepository: StateVolatileRepository;
    fn state_volatile_repository(&self) -> &Self::StateVolatileRepository;
}


#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait AccessTokenRepository: 'static + Sync + Send {
    async fn create(&self, create: &AccessToken) -> Result<(), KernelError>;
    async fn update(&self, update: &AccessToken) -> Result<(), KernelError>;
    async fn delete(&self, delete: &AccessTokenId) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &AccessTokenId) -> Result<Option<AccessToken>, KernelError>;
}

pub trait DependOnAccessTokenRepository: 'static + Sync + Send {
    type AccessTokenRepository: AccessTokenRepository;
    fn access_token_repository(&self) -> &Self::AccessTokenRepository;
}


#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RefreshTokenRepository: 'static + Sync + Send {
    async fn create(&self) -> Result<(), KernelError>;
    async fn delete(&self) -> Result<(), KernelError>;

    async fn find(&self) -> Result<(), KernelError>;
}

pub trait DependOnRefreshTokenRepository: 'static + Sync + Send {
    type RefreshTokenRepository: RefreshTokenRepository;
    fn refresh_token_repository(&self) -> &Self::RefreshTokenRepository;
}