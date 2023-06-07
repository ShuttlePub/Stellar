use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde::de::Error;
use application::transfer::client::{GrantTypeDto, RegisterClientDto, ResponseTypeDto, ScopeDto, TokenEndPointAuthMethodDto};
use kernel::external::Uuid;
use crate::ServerError;

#[allow(unused)]
/// Reference RFC7591
#[derive(Deserialize, Debug)]
pub struct RegistrationForm {
    name: String,
    client_uri: String,
    description: String,
    logo_uri: String,
    tos_uri: String,
    redirect_uris: Vec<String>,
    #[serde(rename = "token_endpoint_auth_method")]
    tepam: TokenEndPointAuthMethod,
    grant_types: Vec<GrantType>,
    response_types: Vec<ResponseType>,
    scopes: Vec<Scope>,
    contacts: Vec<String>,
    policy_uri: String,
    jwks_uri: Option<String>, // ─┬─ MUST NOT both be present in the same request or response.
    jwks: Option<String>,     // ─┘
}

impl RegistrationForm {
    pub fn convert_dto(self, owner: Uuid) -> Result<RegisterClientDto, ServerError> {
        let RegistrationForm {
            name,
            client_uri,
            description,
            logo_uri,
            tos_uri,
            redirect_uris,
            tepam,
            grant_types,
            response_types,
            scopes,
            contacts,
            policy_uri,
            jwks_uri,
            jwks
        } = self;
        Ok(RegisterClientDto {
            name,
            client_uri,
            description,
            logo_uri,
            tos_uri,
            owner_id: owner,
            policy_uri,
            auth_method: tepam.into(),
            grant_types: grant_types.into_iter()
                .map(Into::into)
                .collect(),
            response_types: response_types.into_iter()
                .map(Into::into)
                .collect(),
            redirect_uris,
            scopes: scopes.into_iter()
                .map(Into::into)
                .collect(),
            contacts,
            jwks,
            jwks_uri
        })
    }
}

#[derive(Debug)]
pub enum TokenEndPointAuthMethod {
    ClientSecretPost,
    ClientSecretBasic,
    // OpenID Connect Core 1.0 §9 Client Authentication
    PrivateKeyJWT,
    None,
}

impl Default for TokenEndPointAuthMethod {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for TokenEndPointAuthMethod {
    /// Even if an invalid value exists, 
    /// it is returned as described in the Default Trait, 
    /// so errors can be ignored.
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "client_secret_post" => Self::ClientSecretPost,
            "client_secret_basic" => Self::ClientSecretBasic,
            "private_key_jwt" => Self::PrivateKeyJWT,
            "none" => Self::None,
            _ => Self::default()
        })
    }
}

impl From<TokenEndPointAuthMethod> for TokenEndPointAuthMethodDto {
    fn from(value: TokenEndPointAuthMethod) -> Self {
       match value {
           TokenEndPointAuthMethod::ClientSecretPost => Self::ClientSecretPost,
           TokenEndPointAuthMethod::ClientSecretBasic => Self::ClientSecretBasic,
           TokenEndPointAuthMethod::PrivateKeyJWT => Self::PrivateKeyJWT,
           TokenEndPointAuthMethod::None => Self::None
       }
    }
}

impl<'de> Deserialize<'de> for TokenEndPointAuthMethod {
    //noinspection DuplicatedCode
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where D: serde::Deserializer<'de>
    {
        // See L43 comment.
        Ok(Self::from_str(Deserialize::deserialize(deserializer)?).unwrap())
    }
}

#[derive(Debug)]
pub enum GrantType {
    AuthorizationCode,
    Implicit,
    Password,
    ClientCredentials,
    RefreshToken,
    JWTBearer,
    Saml2Bearer,
}

impl Default for GrantType {
    fn default() -> Self {
        Self::AuthorizationCode
    }
}

impl FromStr for GrantType {
    /// Even if an invalid value exists, 
    /// it is returned as described in the Default Trait, 
    /// so errors can be ignored.
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {            
        Ok(match s {
            "authorization_code" => Self::AuthorizationCode,
            "implicit" => Self::Implicit,
            "password" => Self::Password,
            "client_credentials" => Self::ClientCredentials,
            "refresh_token" => Self::RefreshToken,
            "urn:ietf:params:oauth:grant-type:jwt-bearer" => Self::JWTBearer,
            "urn:ietf:params:oauth:grant-type:saml2-bearer" => Self::Saml2Bearer,
            _ => Self::default() // Here it is.
        })
    }
}

impl From<GrantType> for GrantTypeDto {
    fn from(value: GrantType) -> Self {
        match value {
            GrantType::AuthorizationCode => Self::AuthorizationCode,
            GrantType::Implicit => Self::Implicit,
            GrantType::Password => Self::Password,
            GrantType::ClientCredentials => Self::ClientCredentials,
            GrantType::RefreshToken => Self::RefreshToken,
            GrantType::JWTBearer => Self::JWTBearer,
            GrantType::Saml2Bearer => Self::Saml2Bearer
        }
    }
}

impl<'de> Deserialize<'de> for GrantType {
    //noinspection DuplicatedCode
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where D: serde::Deserializer<'de> 
    {
        // See L74 comment.
        Ok(Self::from_str(Deserialize::deserialize(deserializer)?).unwrap())
    }
}

#[derive(Debug)]
pub enum ResponseType {
    Token,
    Code
}

impl FromStr for ResponseType {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "token" => Self::Token,
            "code" => Self::Code,
            _ => return Err(ServerError::InvalidValue {
                method: "from_str in response type",
                value: s.to_string(),
            })
        })
    }
}

impl From<ResponseType> for ResponseTypeDto {
    fn from(value: ResponseType) -> Self {
       match value {
           ResponseType::Token => Self::Token,
           ResponseType::Code => Self::Code
       }
    }
}

impl<'de> Deserialize<'de> for ResponseType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Self::from_str(Deserialize::deserialize(deserializer)?)
            .map_err(|e| D::Error::custom(e.to_string()))
    }
}

#[derive(Deserialize, Debug)]
pub struct Scope {
    name: String,
    desc: Option<String>,
}

impl From<Scope> for ScopeDto {
    fn from(value: Scope) -> Self {
        Self {
            method: value.name,
            description: value.desc
        }
    }
}