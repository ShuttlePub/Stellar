use kernel::entities::{
    Method,
    DestructUpdateTime,
    AuthorizeToken,
    DestructAuthorizeToken,
    DestructAuthorizeTokenContext, 
    AccessToken, 
    DestructAccessToken, 
    DestructAccessTokenContext,
};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthorizeTokenDto {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub account: Uuid,
    pub client_id: Uuid,
    pub scope: Vec<String>,
    pub redirect_uri: String,
    pub expired_in: OffsetDateTime,
}

impl From<AuthorizeToken> for AuthorizeTokenDto {
    fn from(origin: AuthorizeToken) -> Self {
        let DestructAuthorizeToken {
            id,
            date,
            ctx,
        } = origin.into_destruct();
        let DestructUpdateTime {
            created_at,
            updated_at,
        } = date.into_destruct();
        let DestructAuthorizeTokenContext {
            account,
            client_id,
            scopes,
            redirect_uri,
            expired_in,
        } = ctx.into_destruct();
        Self { 
            id: id.into(),
            created_at: created_at.into(),
            updated_at: updated_at.into(),
            account: account.into(),
            client_id: client_id.into(),
            scope: Vec::from(scopes).into_iter().map(|method: Method| method.into()).collect(),
            redirect_uri: redirect_uri.into(),
            expired_in: expired_in.into()
        }
    }
}

#[derive(Debug)]
pub struct CreateAuthorizeTokenDto {
    pub response_type: String,
    pub client_id: Uuid,
    pub client_secret: Option<String>,
    pub redirect_uri: String,
    pub scope: Vec<String>,
}

impl CreateAuthorizeTokenDto {
    pub fn new(
        response_type: impl Into<String>,
        client_id: impl Into<Uuid>,
        client_secret: impl Into<Option<String>>,
        redirect_uri: impl Into<String>,
        scope: impl Into<Vec<String>>
    ) -> Self {
        Self { 
            response_type: response_type.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
            scope: scope.into()
        }
    }
}

#[derive(Debug)]
pub struct AccessTokenDto {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub client: Uuid,
    pub account: Uuid,
    pub scope: Vec<String>,
    pub issuer: String,
    pub audience: String,
    pub subject: String,
    pub issued_at: OffsetDateTime,
    pub not_before: OffsetDateTime,
    pub expired_in: OffsetDateTime
}

impl From<AccessToken> for AccessTokenDto {
    fn from(origin: AccessToken) -> Self {
        let DestructAccessToken {
            id,
            date,
            ctx,
        } = origin.into_destruct();
        let DestructUpdateTime {
            created_at,
            updated_at,
        } = date.into_destruct();
        let DestructAccessTokenContext {
            scope,
            client_id,
            account,
            exp,
            iat,
            nbf,
            sub,
            aud,
            iss,
        } = ctx.into_destruct();
        Self { 
            id: id.into(),
            created_at: created_at.into(),
            updated_at: updated_at.into(),
            client: client_id.into(),
            account: account.into(), 
            scope: Vec::from(scope).into_iter().map(|method: Method| method.into()).collect(), 
            issuer: iss.into(), 
            audience: aud.into(), 
            subject: sub.into(), 
            issued_at: iat.into(), 
            not_before: nbf.into(), 
            expired_in: exp.into()
        }
    }
}

#[derive(Debug)]
pub struct CreateAccessTokenDto {
    pub grand_type: String,
    pub client_id: Uuid,
    pub redirect_uri: String
}

impl CreateAccessTokenDto {
    pub fn new(
        grand_type: impl Into<String>,
        client_id: impl Into<Uuid>,
        redirect_uri: impl Into<String>
    ) -> Self {
        Self {
            grand_type: grand_type.into(),
            client_id: client_id.into(),
            redirect_uri: redirect_uri.into()
        }
    }
}