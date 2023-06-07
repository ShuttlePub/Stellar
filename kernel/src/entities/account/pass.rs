use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::KernelError;

static ARGON: Lazy<Argon2> = Lazy::new(Argon2::default);

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Password(String);

impl Password {
    pub fn new(pass: impl Into<String>) -> Result<Self, KernelError> {
        let pass: String = pass.into();
        let salt = SaltString::generate(&mut OsRng);
        let pass  = ARGON.hash_password(pass.as_bytes(), &salt)
            .map_err(KernelError::Cryption)?
            .to_string();
        Ok(Self(pass))
    }

    pub fn new_unchecked(pass: impl Into<String>) -> Self {
        Self(pass.into())
    }

    /// Verifies passwords.
    ///
    /// If a match is found, `()` is returned;
    /// if no match is found or an error occurs, [KernelError] is returned.
    ///
    /// For an implementation, See [argon2::password_hash::PasswordVerifier] functions
    pub fn verify(&self, pass: impl Into<String>) -> Result<(), KernelError> {
        let self_hashing = PasswordHash::new(&self.0)
            .map_err(KernelError::Cryption)?;
        ARGON.verify_password(pass.into().as_bytes(), &self_hashing)
            .map_err(|e| {
                println!("{:?}", e);
                KernelError::InvalidPassword(e)
            })?;
        Ok(())
    }
}

impl From<Password> for String {
    fn from(origin: Password) -> Self {
        origin.0
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify() -> anyhow::Result<()> {
        let phrase = "PaSSw0rd";
        let pass = Password::new(phrase)?;

        pass.verify(phrase)?;
        Ok(())
    }
}