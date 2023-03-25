use application::interactor::{
    CreateNonVerifiedAccountInteractor, 
    VerifyAccountInteractor,
    CreateAccountInteractor, 
    UpdateAccountInteractor, 
    DeleteAccountInteractor, 
    RestInteractor, 
};
use driver::{
    DataBaseDriver,
    SmtpDriver, 
    database::{
        AccountDataBase, 
        NonVerifiedAccountDataBase
    }, 
    transport::VerificationMailer
};

use crate::ServerError;

#[allow(dead_code)]
type Handler = RestInteractor<
    CreateNonVerifiedAccountInteractor<NonVerifiedAccountDataBase, VerificationMailer>,
    VerifyAccountInteractor<NonVerifiedAccountDataBase>,
    CreateAccountInteractor<AccountDataBase, NonVerifiedAccountDataBase>,
    UpdateAccountInteractor<AccountDataBase>,
    DeleteAccountInteractor<AccountDataBase>
>;

#[derive(Clone)]
pub struct InteractionHandler {
    handle: Handler
}

impl InteractionHandler {
    pub async fn inject() -> Result<Self, ServerError> {
        let pg_pool = DataBaseDriver::setup_postgres().await?;
        let redis_pool = DataBaseDriver::setup_redis().await?;
        let mailer = SmtpDriver::setup_lettre()?;

        let ac_repo = AccountDataBase::new(pg_pool);
        let nvac_repo = NonVerifiedAccountDataBase::new(redis_pool);
        let verification_mailer = VerificationMailer::new(mailer);

        let nvacc = CreateNonVerifiedAccountInteractor::new(nvac_repo.clone(), verification_mailer);
        let acv = VerifyAccountInteractor::new(nvac_repo.clone());
        let acc = CreateAccountInteractor::new(ac_repo.clone(), nvac_repo);
        let acu = UpdateAccountInteractor::new(ac_repo.clone());
        let acd = DeleteAccountInteractor::new(ac_repo);


        Ok(Self { 
            handle: Handler::new(nvacc, acv, acc, acu, acd)
        })
    }
}

impl AsRef<Handler> for InteractionHandler {
    fn as_ref(&self) -> &Handler {
        &self.handle
    }
}