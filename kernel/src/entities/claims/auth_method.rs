use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub enum TokenEndPointAuthMethod {
    ClientSecretPost,
    ClientSecretBasic,
    None,

    PrivateKeyJWK
}

impl AsRef<str> for TokenEndPointAuthMethod {
    fn as_ref(&self) -> &str {
        match self {
            TokenEndPointAuthMethod::ClientSecretPost => "client_secret_post",
            TokenEndPointAuthMethod::ClientSecretBasic => "client_secret_basic",
            TokenEndPointAuthMethod::None => "none",
            TokenEndPointAuthMethod::PrivateKeyJWK => "private_key_jwk"
        }
    }
}