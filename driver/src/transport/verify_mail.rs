use kernel::{transport::VerificationMailTransporter, entities::{VerificationCode, Address}, KernelError};
use lettre::{Message, message::Mailbox, AsyncTransport};
use once_cell::sync::Lazy;

use crate::{DriverError, SmtpPool};

#[derive(Clone)]
pub struct VerificationMailer {
    mailer: SmtpPool
}

impl VerificationMailer {
    pub fn new(mailer: SmtpPool) -> Self {
        Self { mailer }
    }
}

#[async_trait::async_trait]
impl VerificationMailTransporter for VerificationMailer {
    async fn send(&self, code: &VerificationCode, address: &Address) -> Result<(), KernelError> {
        SmtpInternal::send(code, address, &self.mailer).await?;
        Ok(())
    }
}

pub(in crate::transport) struct SmtpInternal;

static MB: Lazy<Mailbox> = Lazy::new(|| "Stellar <support@shuttle.pub>".parse().expect("cannot parse `MailBox`"));

impl SmtpInternal {
    pub async fn send(code: &VerificationCode, address: &Address, mailer: &SmtpPool) -> Result<(), DriverError> {
        let msg = Message::builder()
            .from(MB.clone())
            .to(address.as_ref().parse()?)
            .subject("Verification Code for Stellar")
            .body(format!("verification code: {}", code.as_ref()))?;

        mailer.send(msg).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use kernel::entities::{VerificationCode, Address};
    use lettre::transport::smtp::authentication::Credentials;

    use crate::SmtpPool;

    use super::SmtpInternal;

    async fn mailer_setup() -> anyhow::Result<SmtpPool> {
        dotenvy::dotenv().ok();
        dotenvy::from_filename("private.env").ok();
        let relay = dotenvy::var("RELAY_SERVER_URL")
            .expect("`RELAY_SERVER_URL` does not set! This value required.");
        let cred_address = dotenvy::var("SMTP_CREDENTIAL_ADDRESS")
            .expect("`SMTP_CREDENTIAL_ADDRESS` does not set! This value required.");
        let cred_pass = dotenvy::var("SMTP_CREDENTIAL_PASSWORD")
            .expect("`SMTP_CREDENTIAL_PASSWORD` does not set! This value required.");
        let cred = Credentials::new(cred_address, cred_pass);
        let mailer = SmtpPool::relay(&relay)?
            .credentials(cred)
            .build();

        Ok(mailer)
    }

    #[ignore = "It does not work as is because it depends on private information."]
    #[tokio::test]
    async fn mailing_test() -> anyhow::Result<()> {
        let mailer = mailer_setup().await?;

        let code = VerificationCode::default();
        let address = Address::new("reirokusanami.rdh@gmail.com");
        SmtpInternal::send(&code, &address, &mailer).await?;

        Ok(())
    }
}