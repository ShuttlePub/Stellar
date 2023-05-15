use application::services::{
    DependOnCreateNonVerifiedAccountService,
    DependOnApproveAccountService,
    DependOnCreateAccountService,
    DependOnDeleteAccountService,
    DependOnUpdateAccountService,
    DependOnRegisterClientService,
    DependOnUpdateClientService
};
use kernel::{
    repository::{
        DependOnAccountRepository,
        DependOnClientRegistry,
        DependOnNonVerifiedAccountRepository
    },
    transporter::{
        DependOnVerificationMailTransporter
    }
};

use application::interactor::{
    RegisterClientInteractor, 
    UpdateClientInteractor
};
use driver::database::{
    AccountDataBase, 
    ClientDataBase, 
    NonVerifiedAccountDataBase
};
use driver::{DataBaseDriver, SmtpDriver};
use driver::transport::VerificationMailer;
use crate::ServerError;

type ClientRegisterer = RegisterClientInteractor<ClientDataBase, AccountDataBase>;

#[derive(Clone)]
pub struct Handler {
    ac_repo: AccountDataBase,
    nvac_repo: NonVerifiedAccountDataBase,
    clients: ClientDataBase,
    mailer: VerificationMailer,

    client_reg: ClientRegisterer,
    client_upd: UpdateClientInteractor<ClientDataBase, AccountDataBase>
}

impl Handler {
    #[allow(dead_code)]
    pub async fn init() -> Result<Self, ServerError> {
        let pg_pool = DataBaseDriver::setup_postgres().await?;
        let redis_pool = DataBaseDriver::setup_redis().await?;
        let smtp_pool = SmtpDriver::setup_lettre()?;

        let ac_repo = AccountDataBase::new(pg_pool.clone());
        let nvac_repo = NonVerifiedAccountDataBase::new(redis_pool);
        let clients = ClientDataBase::new(pg_pool);
        let mailer = VerificationMailer::new(smtp_pool);

        let client_reg = RegisterClientInteractor::new(clients.clone(), ac_repo.clone());
        let client_upd = UpdateClientInteractor::new(clients.clone(), ac_repo.clone());
        Ok(Self {
            ac_repo,
            nvac_repo,
            clients,
            mailer,

            client_reg,
            client_upd
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

impl DependOnClientRegistry for Handler {
    type ClientRegistry = ClientDataBase;

    fn client_registry(&self) -> &Self::ClientRegistry {
        &self.clients
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

    fn delete_account_service(&self) -> &Self::DeleteAccountService {
        self
    }
}

impl DependOnRegisterClientService for Handler {
    type RegisterClientService = ClientRegisterer;

    fn register_client_service(&self) -> &Self::RegisterClientService {
        &self.client_reg
    }
}

impl DependOnUpdateClientService for Handler {
    type UpdateClientService = UpdateClientInteractor<ClientDataBase, AccountDataBase>;

    fn update_client_service(&self) -> &Self::UpdateClientService {
        &self.client_upd
    }
}