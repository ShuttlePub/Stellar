use kernel::entities::{
    Client,
    ClientSecret,
    DestructClient,
    DestructClientId,
    DestructClientSecret,
    TokenEndPointAuthMethod as TokenEndPointAuthMethodDomain,
    GrantType as GrantTypeDomain,
    ResponseType as ResponseTypeDomain,
    ScopeMethod,
    ScopeDescription,
    Jwks
};
use kernel::external::{JsonWebKey, OffsetDateTime, Uuid};

#[derive(Debug)]
pub struct ClientDto {
    pub id: Uuid,
    pub id_iat: OffsetDateTime,
    pub name: String,
    pub client_uri: String,
    pub description: String,
    pub secret: Option<String>,
    pub secret_exp: Option<OffsetDateTime>,
    pub logo_uri: String,
    pub tos_uri: String,
    pub owner_id: Uuid,
    pub policy_uri: String,
    pub auth_method: TokenEndPointAuthMethodDto,
    pub grant_types: Vec<GrantTypeDto>,
    pub response_types: Vec<ResponseTypeDto>,
    pub scopes: Vec<ScopeDto>,
    pub contacts: Vec<String>,
    pub jwks: Option<JwksDto>
}

impl From<Client> for ClientDto {
    fn from(value: Client) -> Self {
        let DestructClient {
            id,
            name,
            uri,
            desc,
            types,
            logo,
            terms,
            owner,
            policy,
            auth_method,
            grant_types,
            response_types,
            scopes,
            contact,
            jwks
        } = value.into_destruct();

        let DestructClientId {
            id,
            issued_at
        } = id.into_destruct();

        let confidential: Option<ClientSecret> = types.into();
        let confidential = match confidential {
            Some(secret) => {
                let DestructClientSecret {
                    secret,
                    expires_at
                } = secret.into_destruct();
                (Some(secret), Some(expires_at))
            }
            None => (None, None)
        };

        Self {
            id,
            id_iat: issued_at,
            name: name.into(),
            client_uri: uri.into(),
            description: desc.into(),
            secret: confidential.0,
            secret_exp: confidential.1,
            logo_uri: logo.into(),
            tos_uri: terms.into(),
            owner_id: owner.into(),
            policy_uri: policy.into(),
            auth_method: auth_method.into(),
            grant_types: grant_types.into_iter()
                .map(Into::into)
                .collect(),
            response_types: response_types.into_iter()
                .map(Into::into)
                .collect(),
            scopes: scopes.into_iter()
                .map(Into::into)
                .collect(),
            contacts: contact.into_iter()
                .map(Into::into)
                .collect(),
            jwks: jwks.map(Into::into),
        }
    }
}

#[derive(Debug)]
pub enum TokenEndPointAuthMethodDto {
    ClientSecretPost,
    ClientSecretBasic,
    None,

    PrivateKeyJWK
}

impl From<TokenEndPointAuthMethodDomain> for TokenEndPointAuthMethodDto {
    fn from(value: TokenEndPointAuthMethodDomain) -> Self {
        match value {
            TokenEndPointAuthMethodDomain::ClientSecretPost => Self::ClientSecretPost,
            TokenEndPointAuthMethodDomain::ClientSecretBasic => Self::ClientSecretBasic,
            TokenEndPointAuthMethodDomain::None => Self::None,
            TokenEndPointAuthMethodDomain::PrivateKeyJWK => Self::PrivateKeyJWK
        }
    }
}

#[derive(Debug)]
pub enum GrantTypeDto {
    AuthorizationCode,
    Implicit,
    Password,
    ClientCredentials,
    RefreshToken,
    JWTBearer,
    Saml2Bearer
}

impl From<GrantTypeDomain> for GrantTypeDto {
    fn from(value: GrantTypeDomain) -> Self {
        match value {
            GrantTypeDomain::AuthorizationCode => Self::AuthorizationCode,
            GrantTypeDomain::Implicit => Self::Implicit,
            GrantTypeDomain::Password => Self::Password,
            GrantTypeDomain::ClientCredentials => Self::ClientCredentials,
            GrantTypeDomain::RefreshToken => Self::RefreshToken,
            GrantTypeDomain::JWTBearer => Self::JWTBearer,
            GrantTypeDomain::Saml2Bearer => Self::Saml2Bearer
        }
    }
}

#[derive(Debug)]
pub enum ResponseTypeDto {
    Code,
    Token
}

impl From<ResponseTypeDomain> for ResponseTypeDto {
    fn from(value: ResponseTypeDomain) -> Self {
        match value {
            ResponseTypeDomain::Code => Self::Code,
            ResponseTypeDomain::Token => Self::Token
        }
    }
}

#[derive(Debug)]
pub struct ScopeDto {
    method: String,
    description: Option<String>
}

impl From<(ScopeMethod, ScopeDescription)> for ScopeDto {
    fn from(value: (ScopeMethod, ScopeDescription)) -> Self {
        Self { method: value.0.into(), description: value.1.into() }
    }
}

#[derive(Debug)]
pub enum JwksDto {
    Uri(String),
    Key(JsonWebKey)
}

impl From<Jwks> for JwksDto {
    fn from(value: Jwks) -> Self {
        match value {
            Jwks::Uri(url) => Self::Uri(url),
            Jwks::Key(key) => Self::Key(key)
        }
    }
}

#[derive(Debug)]
pub struct RegisterClientDto {
    pub id: Uuid,
    pub name: String,
    pub client_uri: String,
    pub description: String,
    pub logo_uri: String,
    pub tos_uri: String,
    pub owner_id: Uuid,
    pub policy_uri: String,
    pub auth_method: TokenEndPointAuthMethodDto,
    pub grant_types: Vec<GrantTypeDto>,
    pub response_types: Vec<ResponseTypeDto>,
    pub scopes: Vec<ScopeDto>,
    pub contacts: Vec<String>,
    pub jwk: JwksDto
}