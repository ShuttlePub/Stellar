mod active;
mod scope;
mod client_id;
mod client_name;
mod exp;
mod iat;
mod nbf;
mod sub;
mod aud;
mod iss;

use std::time::Duration;

pub use self::{
    active::*,
    scope::*,
    client_id::*,
    client_name::*,
    exp::*,
    iat::*,
    nbf::*,
    sub::*,
    aud::*,
    iss::*
};

use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use uuid::Uuid;

use super::update_time::UpdateTime;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, destructure::Destructure)]
pub struct TokenState {
    active: Active,
    scope: Scope,
    client_id: ClientId,
    username: ClientName,
    exp: ExpiredIn,
    iat: IssuedAt,
    nbf: NotBefore,
    sub: Subject,
    aud: Audience,
    iss: Issuer
}

impl TokenState {
    pub fn active(&self) -> &Active {
        &self.active
    }

    pub fn scope(&self) -> &Scope {
        &self.scope
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    pub fn client_name(&self) -> &ClientName {
        &self.username
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
pub struct TokenId(Uuid);

impl TokenId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl From<TokenId> for Uuid {
    fn from(origin: TokenId) -> Self {
        origin.0
    }
}

impl AsRef<Uuid> for TokenId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TokenId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, destructure::Destructure)]
pub struct Token {
    id: TokenId,
    date: UpdateTime,
    state: TokenState
}

impl Token {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<Uuid>,
        linked_client: impl Into<Uuid>,
        user_name: impl Into<String>,
        scoped: impl Into<Vec<String>>,
        issuer: impl Into<String>,
        audience: impl Into<String>,
        subject: impl Into<String>,
        expired_in: impl Into<Duration>,
    ) -> Self {
        Self { 
            id: TokenId::new(id), 
            date: UpdateTime::new(
                OffsetDateTime::now_utc(), 
                OffsetDateTime::now_utc()
            ), 
            state: TokenState { 
                active: Active::default(), 
                scope: Scope::new(scoped), 
                client_id: ClientId::new(linked_client),
                username: ClientName::new(user_name), 
                exp: ExpiredIn::new(expired_in),
                iat: IssuedAt::default(), 
                nbf: NotBefore::default(), 
                sub: Subject::new(subject),
                aud: Audience::new(audience), 
                iss: Issuer::new(issuer)
            }
        }
    }
}