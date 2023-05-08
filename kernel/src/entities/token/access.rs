use std::time::Duration;
use destructure::Destructure;
use rand::distributions::{Alphanumeric, Distribution};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::entities::{ClientId, LoggedAt, ScopeMethod, UserId};

use super::claims::{
    Audience,
    ExpiredIn,
    IssuedAt,
    Issuer,
    NotBefore,
    Subject
};

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

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct AccessTokenContext {
    scope: Vec<ScopeMethod>,
    client_id: ClientId,
    account: UserId,
    aud: Audience,
    exp: ExpiredIn,
    iat: IssuedAt,
    iss: Issuer,
    nbf: NotBefore,
    sub: Subject,
}

impl AccessTokenContext {
    pub fn scope(&self) -> &Vec<ScopeMethod> {
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

impl AccessToken {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        created_at: impl Into<OffsetDateTime>,
        updated_at: impl Into<OffsetDateTime>,
        linked_client: impl Into<Uuid>,
        account: impl Into<Uuid>,
        scoped: impl Into<Vec<ScopeMethod>>,
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
