use base64::{Engine, prelude::BASE64_URL_SAFE};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use crate::KernelError;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct CodeChallenge(Vec<u8>);

impl CodeChallenge {
    pub fn new(code: impl Into<String>) -> Result<Self, KernelError> {
        let code = BASE64_URL_SAFE.decode(code.into())?
            .into_iter()
            .collect::<Vec<u8>>();
        Ok(Self(code))
    }

    pub fn verify(&self, verifier: impl Into<String>) -> Result<(), KernelError> {
        let mut hasher = Sha256::default();
        hasher.update(verifier.into());
        let hashed = hasher.finalize();
        if !self.0.eq(hashed.as_slice()) {
            return Err(KernelError::InvalidValue {
                method: "pkce_code_verify",
                value: format!("{:?}", self.0),
            })
        }
        Ok(())
    }
}

impl From<CodeChallenge> for Vec<u8> {
    fn from(value: CodeChallenge) -> Self {
        value.0
    }
}

impl AsRef<[u8]> for CodeChallenge {
    fn as_ref(&self) -> &[u8] {
       &self.0
    }
}


#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::prelude::BASE64_URL_SAFE;
    use sha2::{Digest, Sha256};
    use crate::entities::volatiles::pkce::CodeChallenge;
    use crate::services::RandomizeService;

    struct TestDomain(String);

    impl TestDomain {
        fn new(value: impl Into<String>) -> Self {
            Self(value.into())
        }
    }

    impl From<TestDomain> for String {
        fn from(value: TestDomain) -> Self {
           value.0
        }
    }

    impl AsRef<str> for TestDomain {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    #[test]
    fn pkce_test() -> anyhow::Result<()> {
        let d: TestDomain = RandomizeService::gen_str(128, TestDomain::new);
        let mut hasher = Sha256::default();
        hasher.update(d.as_ref());
        let hashed = hasher.finalize();
        let encode = BASE64_URL_SAFE.encode(hashed.as_slice());

        let vol = CodeChallenge::new(encode)?;
        vol.verify(d)?;

        Ok(())
    }
}