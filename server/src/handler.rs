use application::services::{
    DependOnCreateNonVerifiedAccountService,
    DependOnApproveAccountService,
    DependOnCreateAccountService,
    DependOnDeleteAccountService, 
    DependOnUpdateAccountService
};
use driver::database::{AccountDataBase, NonVerifiedAccountDataBase};
use driver::{DataBaseDriver, SmtpDriver};
use driver::transport::VerificationMailer;
use kernel::repository::{
    DependOnAccountRepository, 
    DependOnNonVerifiedAccountRepository
};
use kernel::transporter::DependOnVerificationMailTransporter;
use crate::ServerError;

#[derive(Clone)]
pub struct Handler {
    ac_repo: AccountDataBase,
    nvac_repo: NonVerifiedAccountDataBase,
    mailer: VerificationMailer
}

impl Handler {
    #[allow(dead_code)]
    pub async fn init() -> Result<Self, ServerError> {
        let pg_pool = DataBaseDriver::setup_postgres().await?;
        let redis_pool = DataBaseDriver::setup_redis().await?;
        let smtp_pool = SmtpDriver::setup_lettre()?;

        let ac_repo = AccountDataBase::new(pg_pool);
        let nvac_repo = NonVerifiedAccountDataBase::new(redis_pool);
        let mailer = VerificationMailer::new(smtp_pool);

        Ok(Self {
            ac_repo,
            nvac_repo,
            mailer
        })
    }
}

impl DependOnAccountRepository for Handler {
    type AccountRepository = AccountDataBase;

    fn account_repository(&self) -> &Self::AccountRepository {
        &self.ac_repo
    }
}

impl DependOnNonVerifiedAccountRepository for Handler {
    type NonVerifiedAccountRepository = NonVerifiedAccountDataBase;

    fn non_verified_account_repository(&self) -> &Self::NonVerifiedAccountRepository {
        &self.nvac_repo
    }
}

impl DependOnVerificationMailTransporter for Handler {
    type VerificationMailTransporter = VerificationMailer;

    fn verification_mail_transporter(&self) -> &Self::VerificationMailTransporter {
        &self.mailer
    }
}

impl DependOnCreateNonVerifiedAccountService for Handler {
    type CreateNonVerifiedAccountService = Self;

    fn create_non_verified_account_service(&self) -> &Self::CreateNonVerifiedAccountService {
        self
    }
}

impl DependOnApproveAccountService for Handler {
    type ApproveAccountService = Self;

    fn approve_account_service(&self) -> &Self::ApproveAccountService {
       self
    }
}

impl DependOnCreateAccountService for Handler {
    type CreateAccountService = Self;

    fn create_account_service(&self) -> &Self::CreateAccountService {
        self
    }
}

impl DependOnUpdateAccountService for Handler {
    type UpdateAccountService = Self;

    fn update_account_service(&self) -> &Self::UpdateAccountService {
        self
    }
}

impl DependOnDeleteAccountService for Handler {
    type DeleteAccountService = Self;

    fn delete_account_repository(&self) -> &Self::DeleteAccountService {
        self
    }
}