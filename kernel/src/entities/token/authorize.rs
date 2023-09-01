use crate::entities::ResponseType;
use crate::{
    entities::{ClientId, LoggedAt, RedirectUri, ScopeMethod, UserId},
    services::RandomizeService,
    KernelError,
};
use destructure::Destructure;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use try_ref::TryAsRef;
use uuid::Uuid;

use super::claims::ExpiredIn;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct AuthorizeTokenId(String);

impl AuthorizeTokenId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<AuthorizeTokenId> for String {
    fn from(origin: AuthorizeTokenId) -> Self {
        origin.0
    }
}

impl AsRef<str> for AuthorizeTokenId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for AuthorizeTokenId {
    //noinspection DuplicatedCode
    fn default() -> Self {
        RandomizeService::gen_str(128, Self::new)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct TokenOwnedUser(Option<UserId>);

impl TokenOwnedUser {
    pub fn new(id: impl Into<Option<UserId>>) -> Self {
        Self(id.into())
    }
}

impl TryFrom<TokenOwnedUser> for UserId {
    type Error = KernelError;

    fn try_from(value: TokenOwnedUser) -> Result<Self, Self::Error> {
        value.0.ok_or_else(|| KernelError::NotFound {
            method: "try_from",
            entity: "TokenOwnedUser",
            id: "None".to_string(),
        })
    }
}

impl TryAsRef<UserId> for TokenOwnedUser {
    type Error = KernelError;

    fn try_as_ref(&self) -> Result<&UserId, Self::Error> {
        match self.0 {
            Some(ref user) => Ok(user),
            None => Err(KernelError::NotFound {
                method: "try_as_ref",
                entity: "TokenOwnedUser",
                id: "None".to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Destructure)]
pub struct AuthorizeTokenContext {
    client_id: ClientId,
    scopes: Vec<ScopeMethod>,
    response_type: ResponseType,
    redirect_uri: RedirectUri,
    expired_in: ExpiredIn,
}

impl AuthorizeTokenContext {
    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    pub fn scopes(&self) -> &Vec<ScopeMethod> {
        &self.scopes
    }

    pub fn response_type(&self) -> &ResponseType {
        &self.response_type
    }

    pub fn redirect_uri(&self) -> &RedirectUri {
        &self.redirect_uri
    }

    pub fn expired_in(&self) -> &ExpiredIn {
        &self.expired_in
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Destructure)]
pub struct AuthorizeToken {
    id: AuthorizeTokenId,
    date: LoggedAt,
    owned_by: TokenOwnedUser,
    ctx: AuthorizeTokenContext,
}

impl AuthorizeToken {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        created_at: impl Into<OffsetDateTime>,
        updated_at: impl Into<OffsetDateTime>,
        owned_by: impl Into<Option<Uuid>>,
        client_id: impl Into<Uuid>,
        scope: impl Into<Vec<ScopeMethod>>,
        response_type: impl Into<ResponseType>,
        redirect_uri: impl Into<String>,
        expired_in: impl Into<Duration>,
    ) -> Self {
        Self {
            id: AuthorizeTokenId::new(id),
            owned_by: TokenOwnedUser::new(owned_by.into().map(UserId::new)),
            date: LoggedAt::new(created_at, updated_at),
            ctx: AuthorizeTokenContext {
                client_id: ClientId::new_at_now(client_id),
                scopes: scope.into(),
                response_type: response_type.into(),
                redirect_uri: RedirectUri::new(redirect_uri),
                expired_in: ExpiredIn::new(expired_in),
            },
        }
    }

    pub fn id(&self) -> &AuthorizeTokenId {
        &self.id
    }

    pub fn date(&self) -> &LoggedAt {
        &self.date
    }

    pub fn context(&self) -> &AuthorizeTokenContext {
        &self.ctx
    }
}
