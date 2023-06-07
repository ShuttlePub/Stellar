use kernel::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("cannot find `{id}:{entity}` in the following {method}.")]
    NotFound {
        method: &'static str,
        entity: &'static str,
        id: String
    },
    #[error("invalid value `{value}` in the following {method}.")]
    InvalidValue {
        method: &'static str,
        value: String
    },
    #[error("failed verification `{id}:{entity}` in following {method}.")]
    Verification {
        method: &'static str,
        entity: &'static str,
        id: String,
    },
    #[error(transparent)]
    Authorization(#[from] ExpectedAuthorizationError),
    #[error("require user action.")]
    RequireUserAction(ExpectUserAction),
    #[error(transparent)]
    Other(anyhow::Error),
    #[error(transparent)]
    External(anyhow::Error)
}

#[derive(Debug)]
pub enum ExpectUserAction {
    Login,
    MFA
}

// Todo: Replace the errors assumed in RFC6749 with this one.
#[derive(Debug, thiserror::Error)]
pub enum ExpectedAuthorizationError {
    #[error("")]
    InvalidRequest {
        entity: &'static str,
        value: String
    },
    #[error("")]
    UnAuthorizedClient {
        entity: &'static str,
        id: String
    },
    #[error("")]
    AccessDenied {
        side: &'static str,
        value: String
    },
    #[error("")]
    UnSupportedResponseType {

    },
    #[error("")]
    InvalidScope,
    #[error("")]
    ServerError,
    #[error("")]
    TemporaryUnAvailable
}

impl From<KernelError> for ApplicationError {
    fn from(e: KernelError) -> Self {
        match e {
            KernelError::NotFound { method, entity, id }
                => ApplicationError::NotFound { method, entity, id },
            KernelError::InvalidValue { method, value }
                => ApplicationError::InvalidValue { method, value },
            KernelError::Driver(not_here)
                => ApplicationError::Other(not_here),
            KernelError::External(e)
                => ApplicationError::External(e),
            KernelError::InvalidPassword(e)
                => ApplicationError::Other(anyhow::Error::new(e)),
            KernelError::Cryption(e)
                => ApplicationError::Other(anyhow::Error::new(e)),
            KernelError::JsonWebToken(e)
                => ApplicationError::Other(e),
            KernelError::Base64Decode(e)
                => ApplicationError::Other(e),
            KernelError::Serde(e)
                => ApplicationError::Other(e),
            KernelError::Parse(e)
                => ApplicationError::Other(e),
        }
    }
}