use std::str::FromStr;

use serde::Deserialize;

#[allow(unused)]
/// Reference RFC7591
#[derive(Deserialize, Debug)]
pub struct RegistrationForm {
    redirect_uris: Vec<String>,
    #[serde(rename = "token_endpoint_auth_method")]
    tepam: TokenEndPointAuthMethod,
    grant_types: Vec<GrantTypes>,
    response_types: Vec<String>,
    client_name: String,
    client_uri: String,
    logo_uri: String,
    scopes: Vec<Scope>,
    contacts: Vec<String>,
    tos_uri: String,
    policy_uri: String,
    jwks_uri: String, // ───┬─ Fixme: MUST NOT both be present in the same request or response.
    jwks: String,     // ───┘
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
pub enum GrantTypes {
    AuthorizationCode,
    Implicit,
    Password,
    ClientCredentials,
    RefreshToken,
    JWTBearer,
    Saml2Bearer,
}

impl Default for GrantTypes {
    fn default() -> Self {
        Self::AuthorizationCode
    }
}

impl FromStr for GrantTypes {
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

impl<'de> Deserialize<'de> for GrantTypes {
    //noinspection DuplicatedCode
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where D: serde::Deserializer<'de> 
    {
        // See L74 comment.
        Ok(Self::from_str(Deserialize::deserialize(deserializer)?).unwrap())
    }
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct Scope {
    name: String,
    desc: Option<String>,
}