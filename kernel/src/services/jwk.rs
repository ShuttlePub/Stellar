use crate::entities::Jwks;
use crate::KernelError;

pub struct JwkSelectionService;

impl JwkSelectionService {
    pub fn check(
        key: impl Into<Option<String>>,
        uri: impl Into<Option<String>>
    ) -> Result<Option<Jwks>, KernelError> {
        let key = key.into();
        let uri = uri.into();

        if key.is_some() && uri.is_some() {
            return Err(KernelError::InvalidValue {
                method: "Jwk resource check",
                value: "`jwks` and `jwks_uri` must not be defined at the same time.".to_string(),
            })
        }

        if key.is_some() && uri.is_none() {
            return key.map(Jwks::new).transpose()
        }

        if key.is_none() && uri.is_some() {
            return uri.map(Jwks::new).transpose()
        }

        Ok(None)
    }
}