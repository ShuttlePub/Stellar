use std::time::Duration;

use destructure::Destructure;
use rand::{distributions::Alphanumeric, prelude::Distribution};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use uuid::Uuid;

use super::{
    account::UserId,
    client::{
        ClientId,
        Scopes,
        RedirectUri,
    },
    time::LoggedAt
};


mod aud;
mod exp;
mod iat;
mod iss;
mod nbf;
mod sub;

pub use self::{
    aud::*,
    exp::*,
    iat::*,
    iss::*,
    nbf::*,
    sub::*
};

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct AccessTokenContext {
    scope: Scopes,
    client_id: ClientId,
    account: UserId,
    exp: ExpiredIn,
    iat: IssuedAt,
    nbf: NotBefore,
    sub: Subject,
    aud: Audience,
    iss: Issuer
}

impl AccessTokenContext {
    pub fn scope(&self) -> &Scopes {
        &self.scope
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    pub fn expired_in(&self) -> &ExpiredIn {
        &self.exp
    }

    pub fn issued_at(&self) -> &IssuedAt {
        &self.iat
    }

    pub fn not_before(&self) -> &NotBefore {
        &self.nbf
    }

    pub fn subject(&self) -> &Subject {
        &self.sub
    }

    pub fn audience(&self) -> &Audience {
        &self.aud
    }

    pub fn issuer(&self) -> &Issuer {
        &self.iss
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccessTokenId(String);

impl AccessTokenId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<AccessTokenId> for String {
    fn from(origin: AccessTokenId) -> Self {
        origin.0
    }
}

impl AsRef<str> for AccessTokenId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for AccessTokenId {
    //noinspection DuplicatedCode
    fn default() -> Self {
        let id = Alphanumeric.sample_iter(&mut rand::thread_rng())
            .take(32)
            .map(char::from)
            .collect::<String>();
        Self::new(id)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct AccessToken {
    id: AccessTokenId,
    date: LoggedAt,
    ctx: AccessTokenContext
}

impl AccessToken {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        created_at: impl Into<OffsetDateTime>,
        updated_at: impl Into<OffsetDateTime>,
        linked_client: impl Into<Uuid>,
        account: impl Into<Uuid>,
        scoped: impl Into<Scopes>,
        issuer: impl Into<String>,
        audience: impl Into<String>,
        subject: impl Into<String>,
        expired_in: impl Into<Duration>,
    ) -> Self {
        Self {
            id: AccessTokenId::new(id),
            date: LoggedAt::new(
                created_at,
                updated_at
            ),
            ctx: AccessTokenContext {
                scope: scoped.into(),
                client_id: ClientId::new_at_now(linked_client),
                account: UserId::new(account),
                exp: ExpiredIn::new(expired_in),
                iat: IssuedAt::default(),
                nbf: NotBefore::default(),
                sub: Subject::new(subject),
                aud: Audience::new(audience),
                iss: Issuer::new(issuer)
            }
        }
    }

    pub fn id(&self) -> &AccessTokenId {
        &self.id
    }

    pub fn date(&self) -> &LoggedAt {
        &self.date
    }

    pub fn context(&self) -> &AccessTokenContext {
        &self.ctx
    }
}

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
        let id = Alphanumeric.sample_iter(&mut rand::thread_rng())
            .take(32)
            .map(char::from)
            .collect::<String>();
        Self::new(id)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct AuthorizeTokenContext {
    account: UserId,
    client_id: ClientId,
    scopes: Scopes,
    redirect_uri: RedirectUri,
    expired_in: ExpiredIn,
}

impl AuthorizeTokenContext {
    pub fn account(&self) -> &UserId {
        &self.account
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }
    
    pub fn scopes(&self) -> &Scopes {
        &self.scopes
    }

    pub fn redirect_uri(&self) -> &RedirectUri {
        &self.redirect_uri
    }

    pub fn expired_in(&self) -> &ExpiredIn {
        &self.expired_in
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct AuthorizeToken {
    id: AuthorizeTokenId,
    date: LoggedAt,
    ctx: AuthorizeTokenContext
}

impl AuthorizeToken {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        created_at: impl Into<OffsetDateTime>,
        updated_at: impl Into<OffsetDateTime>,
        account: impl Into<Uuid>,
        client_id: impl Into<Uuid>,
        scope: impl Into<Scopes>,
        redirect_uri: impl Into<String>,
        expired_in: impl Into<Duration>
    ) -> Self {
        Self { 
            id: AuthorizeTokenId::new(id),
            date: LoggedAt::new(
                created_at,
                updated_at
            ),
            ctx: AuthorizeTokenContext { 
                account: UserId::new(account),
                client_id: ClientId::new_at_now(client_id),
                scopes: scope.into(),
                redirect_uri: RedirectUri::new(redirect_uri),
                expired_in: ExpiredIn::new(expired_in)
            }
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