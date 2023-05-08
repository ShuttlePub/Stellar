use std::time::Duration;
use destructure::Destructure;
use rand::distributions::{Alphanumeric, Distribution};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::entities::{ClientId, LoggedAt, RedirectUri, Scopes, UserId};

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
