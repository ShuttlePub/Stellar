#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("cannot find `{id}:{entity}` in the following {method}.")]
    NotFound {
        method: &'static str,
        entity: &'static str,
        id: String,
    },
    #[error("invalid value `{value}` in the following {method}.")]
    InvalidValue {
        method: &'static str,
        value: String
    },
    #[error(transparent)]
    JsonWebToken(anyhow::Error),
    #[error(transparent)]
    Base64Decode(anyhow::Error),
    #[error("failed cryption in argon password hashing. : {0:?}")]
    Cryption(argon2::password_hash::Error),
    #[error("invalid password ")]
    InvalidPassword(argon2::password_hash::Error),
    #[error(transparent)]
    Driver(anyhow::Error),
    #[error(transparent)]
    External(anyhow::Error)
}

impl From<jsonwebtoken::errors::Error> for KernelError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::JsonWebToken(anyhow::Error::new(value))
    }
}

impl From<base64::DecodeError> for KernelError {
    fn from(value: base64::DecodeError) -> Self {
       Self::Base64Decode(anyhow::Error::new(value))
    }
}