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
    #[error("failed cryption in argon password hasing. : {0:?}")]
    Cryption(argon2::password_hash::Error),
    #[error("invalid password ")]
    InvalidPassword(argon2::password_hash::Error),
    #[error(transparent)]
    Driver(anyhow::Error),
    #[error(transparent)]
    External(anyhow::Error)
}