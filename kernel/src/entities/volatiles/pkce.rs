use crate::KernelError;
use base64::{prelude::BASE64_URL_SAFE, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct CodeChallenge(Vec<u8>);

impl CodeChallenge {
    /// Decodes and initializes the given string in **Base64Url** format.
    ///
    /// - Note that the value inside is `Vec<u8>`.
    /// - If you do not need the decoding process,
    ///   consider using `From<Vec<u8>>` instead.
    pub fn new(code: impl Into<String>) -> Result<Self, KernelError> {
        let code = BASE64_URL_SAFE
            .decode(code.into())?
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
            });
        }
        Ok(())
    }
}

impl From<Vec<u8>> for CodeChallenge {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
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
    use crate::entities::volatiles::pkce::CodeChallenge;
    use crate::services::RandomizeService;
    use base64::prelude::BASE64_URL_SAFE;
    use base64::Engine;
    use sha2::{Digest, Sha256};

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
        println!("origin: {}", d.as_ref());
        let mut hasher = Sha256::default();
        hasher.update(d.as_ref());
        let hashed = hasher.finalize();
        let encode = BASE64_URL_SAFE.encode(hashed.as_slice());

        println!("encode: {}", &encode);

        let vol = CodeChallenge::new(encode)?;
        vol.verify(d)?;

        Ok(())
    }
}
