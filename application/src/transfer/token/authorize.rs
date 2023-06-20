use kernel::prelude::entities::{AuthorizeToken, DestructAuthorizeToken, DestructAuthorizeTokenContext};
use kernel::external::Uuid;

#[derive(Debug)]
pub struct AuthorizeTokenDto {
    pub token_id: String,
    pub token_type: String,
    pub response_type: String,
    pub redirect_uri: String,
    pub scope: Vec<String>,
    pub state: String,
}

impl AuthorizeTokenDto {
    pub fn from_with(
        origin: AuthorizeToken,
        token_type: impl Into<String>,
        state: impl Into<String>
    ) -> Self {
        let DestructAuthorizeToken {
            id,
            ctx,
            ..
        } = origin.into_destruct();
        let DestructAuthorizeTokenContext {
            scopes,
            response_type,
            redirect_uri,
            ..
        } = ctx.into_destruct();

        Self {
            token_id: id.into(),
            token_type: token_type.into(),
            response_type: response_type.into(),
            redirect_uri: redirect_uri.as_ref().to_owned(),
            scope: scopes.into_iter().map(Into::into).collect(),
            state: state.into(),
        }
    }
}

#[derive(Debug)]
pub struct CreateAuthorizeTokenDto {
    pub response_type: String,
    pub client_id: Uuid,
    pub redirect_uri: Option<String>,
    pub scope: Vec<String>,
    pub state: String,
    pub code_challenge: String,
    pub code_challenge_method: String
}

#[derive(Debug)]
pub struct AcceptUserFormDto {
    pub address: String,
    pub pass: String
}