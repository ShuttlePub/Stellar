use serde::Deserialize;


#[allow(unused)]
/// Reference RFC7591
#[derive(Deserialize, Debug)]
pub struct RegistrationForm {
    response_code: Vec<String>,
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
    software_id: String,
    software_version: String
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

impl<'de> Deserialize<'de> for TokenEndPointAuthMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where D: serde::Deserializer<'de>
    {
        Ok(match Deserialize::deserialize(deserializer)? {
            "client_secret_post" => Self::ClientSecretPost,
            "client_secret_basic" => Self::ClientSecretBasic,
            "private_key_jwt" => Self::PrivateKeyJWT,
            "none" => Self::None,
            _ => Self::default()
         })
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

impl<'de> Deserialize<'de> for GrantTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where D: serde::Deserializer<'de> 
    {   
        Ok(match Deserialize::deserialize(deserializer)? {
            "authorization_code" => Self::AuthorizationCode,
            "implicit" => Self::Implicit,
            "password" => Self::Password,
            "client_credentials" => Self::ClientCredentials,
            "refresh_token" => Self::RefreshToken,
            "urn:ietf:params:oauth:grant-type:jwt-bearer" => Self::JWTBearer,
            "urn:ietf:params:oauth:grant-type:saml2-bearer" => Self::Saml2Bearer,
            _ => Self::default()
        })
    }
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct Scope {
    name: String,
    desc: Option<String>,
}