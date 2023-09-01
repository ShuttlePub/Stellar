use crate::transfer::mfa_code::TicketIdDto;
use crate::transfer::token::{AcceptUserFormDto, AuthorizeTokenDto, CreateAuthorizeTokenDto};
use crate::ApplicationError;
use kernel::{
    external::{Duration, OffsetDateTime},
    interfaces::repository::{
        AccountRepository, AuthorizeTokenRepository, ClientRegistry, DependOnAccessTokenRepository,
        DependOnAccountRepository, DependOnAuthorizeTokenRepository, DependOnClientRegistry,
        DependOnPKCEVolatileRepository, DependOnPendingAuthorizeTokenRepository,
        DependOnRefreshTokenRepository, DependOnStateVolatileRepository, PKCEVolatileRepository,
        PendingAuthorizeTokenRepository, StateVolatileRepository,
    },
    prelude::entities::{
        Address, AuthorizeToken, AuthorizeTokenId, ClientId, CodeChallenge, DestructAccount,
        DestructClient, ResponseType, ScopeMethod, State, TicketId, TokenOwnedUser,
    },
};

#[async_trait::async_trait]
pub trait PendingAuthorizeTokenService:
    'static
    + Sync
    + Send
    + DependOnClientRegistry
    + DependOnPKCEVolatileRepository
    + DependOnPendingAuthorizeTokenRepository
    + DependOnStateVolatileRepository
{
    //noinspection DuplicatedCode
    async fn pending(
        &self,
        create: CreateAuthorizeTokenDto,
    ) -> Result<TicketIdDto, ApplicationError> {
        let CreateAuthorizeTokenDto {
            response_type,
            client_id,
            redirect_uri,
            scope,
            state,
            code_challenge,
            code_challenge_method,
        } = create;

        let client_id = ClientId::new_at_now(client_id);

        let Some(client) = self.client_registry().find_by_id(&client_id).await? else {
            return Err(ApplicationError::NotFound {
                method: "find_by_id",
                entity: "client",
                id: client_id.to_string(),
            });
        };

        let code_challenge = CodeChallenge::new(code_challenge)?;

        // There is no advantage to ignoring the PKCE, so it is always required
        if code_challenge_method.ne("S256") {
            return Err(ApplicationError::InvalidValue {
                method: "code_challenge_method validation",
                value: "code_challenge_method required `S256`.".to_string(),
            });
        }

        // https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1
        if response_type.ne("code") {
            return Err(ApplicationError::InvalidValue {
                method: "invalid_request",
                value: format!(
                    "`response_type` must set `code`. invalid {}.",
                    response_type
                ),
            });
        }

        let response_type = ResponseType::Code;

        if client
            .response_types()
            .iter()
            .any(|ty| ty.ne(&response_type))
        {
            return Err(ApplicationError::InvalidValue {
                method: "unsupported_response_type",
                value: "client not support this response_type".to_string(),
            });
        }

        let DestructClient { redirect_uris, .. } = client.into_destruct();
        let redirect_uri = match redirect_uri {
            Some(uri) => redirect_uris
                .into_iter()
                .find(|reg| reg.eq(uri.as_str()))
                .ok_or_else(|| ApplicationError::InvalidValue {
                    method: "redirect_uri validate",
                    value: "The specified uri is not registered with this client.".to_string(),
                })?,
            None => redirect_uris.take_one()?,
        };

        let token_id = AuthorizeTokenId::default();

        let state = State::new(state);

        self.pkce_volatile_repository()
            .save(&token_id, &code_challenge)
            .await?;

        let created_at = OffsetDateTime::now_utc();
        let updated_at = created_at;
        let expired_in = Duration::new(60 * 10, 0);

        let scope = scope
            .into_iter()
            .map(ScopeMethod::new)
            .collect::<Vec<ScopeMethod>>();

        let token = AuthorizeToken::new(
            token_id,
            created_at,
            updated_at,
            None,
            client_id,
            scope,
            response_type,
            redirect_uri,
            expired_in,
        );

        let ticket = TicketId::default();
        self.state_volatile_repository()
            .save(&ticket, &state)
            .await?;
        self.pending_authorize_token_repository()
            .save(&ticket, &token)
            .await?;

        Ok(ticket.into())
    }
}

pub trait DependOnPendingAuthorizeTokenService: 'static + Sync + Send {
    type PendingAuthorizeTokenService: PendingAuthorizeTokenService;
    fn pending_authorize_token_service(&self) -> &Self::PendingAuthorizeTokenService;
}

#[async_trait::async_trait]
pub trait AcceptAuthorizeTokenService:
    'static
    + Sync
    + Send
    + DependOnAccountRepository
    + DependOnStateVolatileRepository
    + DependOnPendingAuthorizeTokenRepository
    + DependOnAuthorizeTokenRepository
{
    async fn accept(
        &self,
        ticket: &str,
        state: &str,
        accept: AcceptUserFormDto,
    ) -> Result<AuthorizeTokenDto, ApplicationError> {
        let ticket = TicketId::new(ticket);
        let Some(token) = self
            .pending_authorize_token_repository()
            .find(&ticket)
            .await?
        else {
            return Err(ApplicationError::NotFound {
                method: "find",
                entity: "ticket",
                id: format!("Ticket not found or expired, ticket: {:?}", ticket),
            });
        };

        let Some(origin) = self.state_volatile_repository().find(&ticket).await? else {
            return Err(ApplicationError::NotFound {
                method: "find",
                entity: "state",
                id: ticket.as_ref().to_string(),
            });
        };

        if origin.ne(state) {
            return Err(ApplicationError::InvalidValue {
                method: "state_eq",
                value: state.to_string(),
            });
        }

        let AcceptUserFormDto { address, pass } = accept;

        let address = Address::new(address);

        let Some(account) = self.account_repository().find_by_address(&address).await? else {
            return Err(ApplicationError::NotFound {
                method: "find_by_address",
                entity: "account",
                id: format!("account not found. adr: {:?}", address),
            });
        };

        account.pass().verify(pass)?;

        let DestructAccount { id, .. } = account.into_destruct();

        let mut token = token.into_destruct();

        token.owned_by = TokenOwnedUser::new(id);

        let token = token.freeze();

        self.pending_authorize_token_repository()
            .dele(&ticket)
            .await?;

        self.authorize_token_repository()
            .save(token.id(), &token)
            .await?;

        Ok(AuthorizeTokenDto::from_with(token, "bearer", state))
    }
}

pub trait DependOnAcceptAuthorizeTokenService: 'static + Sync + Send {
    type AcceptAuthorizeTokenService: AcceptAuthorizeTokenService;
    fn accept_authorize_token_service(&self) -> &Self::AcceptAuthorizeTokenService;
}

#[async_trait::async_trait]
pub trait RejectAuthorizeTokenService:
    'static
    + Sync
    + Send
    + DependOnPendingAuthorizeTokenRepository
    + DependOnStateVolatileRepository
    + DependOnPKCEVolatileRepository
{
    async fn reject(&self, ticket: &str) -> Result<(), ApplicationError> {
        let ticket = TicketId::new(ticket);
        if let Some(token) = self
            .pending_authorize_token_repository()
            .find(&ticket)
            .await?
        {
            self.pkce_volatile_repository().dele(token.id()).await?;
        }
        self.state_volatile_repository().dele(&ticket).await?;
        self.pending_authorize_token_repository()
            .dele(&ticket)
            .await?;
        Ok(())
    }
}

pub trait DependOnRejectAuthorizeTokenService: 'static + Sync + Send {
    type RejectAuthorizeTokenService: RejectAuthorizeTokenService;
    fn reject_authorize_token_service(&self) -> &Self::RejectAuthorizeTokenService;
}

#[async_trait::async_trait]
pub trait CreateAccessTokenService:
    'static
    + Sync
    + Send
    + DependOnPKCEVolatileRepository
    + DependOnAccessTokenRepository
    + DependOnRefreshTokenRepository
{
    async fn create(&self) -> Result<(), ApplicationError>;
}

#[async_trait::async_trait]
pub trait RefreshAccessTokenService:
    'static + Sync + Send + DependOnRefreshTokenRepository + DependOnAccessTokenRepository
{
    async fn refresh(&self) -> Result<(), ApplicationError>;
}
