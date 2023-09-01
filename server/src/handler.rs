use application::{
    interactor::{RegisterClientInteractor, UpdateClientInteractor},
    services::{
        DependOnAcceptAuthorizeTokenService, DependOnCreateAccountService,
        DependOnCreateNonVerifiedAccountService, DependOnDeleteAccountService,
        DependOnPendingAuthorizeTokenService, DependOnRegisterClientService,
        DependOnRejectAuthorizeTokenService, DependOnUpdateAccountService,
        DependOnUpdateClientService, DependOnVerifyAccountService, DependOnVerifyMFACodeService,
    },
};
use kernel::interfaces::{
    repository::{
        DependOnAcceptedActionVolatileRepository, DependOnAccountRepository,
        DependOnAuthorizeTokenRepository, DependOnClientRegistry,
        DependOnMFACodeVolatileRepository, DependOnPKCEVolatileRepository,
        DependOnPendingActionVolatileRepository, DependOnPendingAuthorizeTokenRepository,
        DependOnSessionVolatileRepository, DependOnStateVolatileRepository,
        DependOnTemporaryAccountRepository,
    },
    transport::DependOnVerificationMailTransporter,
};

use crate::ServerError;
#[allow(unused_imports)]
use driver::{
    database::{
        AcceptedActionVolatileDataBase, AccountDataBase, AuthorizeTokenVolatileDataBase,
        ClientDataBase, MFACodeVolatileDataBase, NonVerifiedAccountDataBase, PKCEVolatileDataBase,
        PendingActionVolatileDataBase, PendingAuthorizeTokenVolatileDataBase,
        SessionVolatileDataBase, StateVolatileDataBase,
    },
    transport::VerificationMailer,
    DataBaseDriver, SmtpDriver,
};

#[cfg(debug_assertions)]
use self::mock::MockVerificationMailer;

type ClientRegisterer = RegisterClientInteractor<ClientDataBase, AccountDataBase>;

#[derive(Clone)]
pub struct Handler {
    ac_repo: AccountDataBase,
    clients: ClientDataBase,

    nvac_repo: NonVerifiedAccountDataBase,
    p_authz_v_repo: PendingAuthorizeTokenVolatileDataBase,
    authz_v_repo: AuthorizeTokenVolatileDataBase,
    pkce_v_repo: PKCEVolatileDataBase,
    state_v_repo: StateVolatileDataBase,
    mfa_code_v_repo: MFACodeVolatileDataBase,
    session_v_repo: SessionVolatileDataBase,
    pending_action_v_repo: PendingActionVolatileDataBase,
    accepted_action_v_repo: AcceptedActionVolatileDataBase,

    #[cfg(not(debug_assertions))]
    mailer: VerificationMailer,

    #[cfg(debug_assertions)]
    mailer: MockVerificationMailer,

    client_reg: ClientRegisterer,
    client_upd: UpdateClientInteractor<ClientDataBase, AccountDataBase>,
}

impl Handler {
    #[allow(dead_code)]
    pub async fn init() -> Result<Self, ServerError> {
        let pg_pool = DataBaseDriver::setup_postgres().await?;
        let redis_pool = DataBaseDriver::setup_redis().await?;

        #[cfg(not(debug_assertions))]
        let smtp_pool = SmtpDriver::setup_lettre()?;

        let ac_repo = AccountDataBase::new(pg_pool.clone());
        let clients = ClientDataBase::new(pg_pool);

        let nvac_repo = NonVerifiedAccountDataBase::new(redis_pool.clone());
        let p_authz_v_repo = PendingAuthorizeTokenVolatileDataBase::new(redis_pool.clone());
        let authz_v_repo = AuthorizeTokenVolatileDataBase::new(redis_pool.clone());
        let pkce_v_repo = PKCEVolatileDataBase::new(redis_pool.clone());
        let state_v_repo = StateVolatileDataBase::new(redis_pool.clone());
        let mfa_code_v_repo = MFACodeVolatileDataBase::new(redis_pool.clone());
        let session_v_repo = SessionVolatileDataBase::new(redis_pool.clone());
        let pending_action_v_repo = PendingActionVolatileDataBase::new(redis_pool.clone());
        let accepted_action_v_repo = AcceptedActionVolatileDataBase::new(redis_pool);

        #[cfg(not(debug_assertions))]
        let mailer = VerificationMailer::new(smtp_pool);

        #[cfg(debug_assertions)]
        let mailer = MockVerificationMailer::new();

        let client_reg = RegisterClientInteractor::new(clients.clone(), ac_repo.clone());
        let client_upd = UpdateClientInteractor::new(clients.clone(), ac_repo.clone());

        Ok(Self {
            ac_repo,
            clients,

            nvac_repo,
            p_authz_v_repo,
            authz_v_repo,
            pkce_v_repo,
            state_v_repo,
            mfa_code_v_repo,
            session_v_repo,
            pending_action_v_repo,
            accepted_action_v_repo,

            mailer,

            client_reg,
            client_upd,
        })
    }
}

impl DependOnAccountRepository for Handler {
    type AccountRepository = AccountDataBase;

    fn account_repository(&self) -> &Self::AccountRepository {
        &self.ac_repo
    }
}

impl DependOnClientRegistry for Handler {
    type ClientRegistry = ClientDataBase;

    fn client_registry(&self) -> &Self::ClientRegistry {
        &self.clients
    }
}

impl DependOnTemporaryAccountRepository for Handler {
    type TemporaryAccountRepository = NonVerifiedAccountDataBase;

    fn temporary_account_repository(&self) -> &Self::TemporaryAccountRepository {
        &self.nvac_repo
    }
}

impl DependOnPKCEVolatileRepository for Handler {
    type PKCEVolatileRepository = PKCEVolatileDataBase;
    fn pkce_volatile_repository(&self) -> &Self::PKCEVolatileRepository {
        &self.pkce_v_repo
    }
}

impl DependOnStateVolatileRepository for Handler {
    type StateVolatileRepository = StateVolatileDataBase;
    fn state_volatile_repository(&self) -> &Self::StateVolatileRepository {
        &self.state_v_repo
    }
}

impl DependOnPendingAuthorizeTokenRepository for Handler {
    type PendingAuthorizeTokenRepository = PendingAuthorizeTokenVolatileDataBase;
    fn pending_authorize_token_repository(&self) -> &Self::PendingAuthorizeTokenRepository {
        &self.p_authz_v_repo
    }
}

impl DependOnMFACodeVolatileRepository for Handler {
    type MFACodeVolatileRepository = MFACodeVolatileDataBase;
    fn mfa_code_volatile_repository(&self) -> &Self::MFACodeVolatileRepository {
        &self.mfa_code_v_repo
    }
}

impl DependOnSessionVolatileRepository for Handler {
    type SessionVolatileRepository = SessionVolatileDataBase;
    fn session_volatile_repository(&self) -> &Self::SessionVolatileRepository {
        &self.session_v_repo
    }
}

impl DependOnPendingActionVolatileRepository for Handler {
    type PendingActionVolatileRepository = PendingActionVolatileDataBase;
    fn pending_action_volatile_repository(&self) -> &Self::PendingActionVolatileRepository {
        &self.pending_action_v_repo
    }
}

impl DependOnAcceptedActionVolatileRepository for Handler {
    type AcceptedActionVolatileRepository = AcceptedActionVolatileDataBase;
    fn accepted_action_volatile_repository(&self) -> &Self::AcceptedActionVolatileRepository {
        &self.accepted_action_v_repo
    }
}

impl DependOnAuthorizeTokenRepository for Handler {
    type AuthorizeTokenRepository = AuthorizeTokenVolatileDataBase;
    fn authorize_token_repository(&self) -> &Self::AuthorizeTokenRepository {
        &self.authz_v_repo
    }
}

impl DependOnCreateNonVerifiedAccountService for Handler {
    type CreateNonVerifiedAccountService = Self;

    fn create_non_verified_account_service(&self) -> &Self::CreateNonVerifiedAccountService {
        self
    }
}

#[cfg(not(debug_assertions))]
impl DependOnVerificationMailTransporter for Handler {
    type VerificationMailTransporter = VerificationMailer;

    fn verification_mail_transporter(&self) -> &Self::VerificationMailTransporter {
        &self.mailer
    }
}

#[cfg(debug_assertions)]
impl DependOnVerificationMailTransporter for Handler {
    type VerificationMailTransporter = MockVerificationMailer;
    fn verification_mail_transporter(&self) -> &Self::VerificationMailTransporter {
        &self.mailer
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

impl DependOnVerifyAccountService for Handler {
    type VerifyAccountService = Self;
    fn verify_account_service(&self) -> &Self::VerifyAccountService {
        self
    }
}

impl DependOnVerifyMFACodeService for Handler {
    type VerifyMFACodeService = Self;
    fn verify_mfa_code_service(&self) -> &Self::VerifyMFACodeService {
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

impl DependOnPendingAuthorizeTokenService for Handler {
    type PendingAuthorizeTokenService = Self;
    fn pending_authorize_token_service(&self) -> &Self::PendingAuthorizeTokenService {
        self
    }
}

impl DependOnAcceptAuthorizeTokenService for Handler {
    type AcceptAuthorizeTokenService = Self;
    fn accept_authorize_token_service(&self) -> &Self::AcceptAuthorizeTokenService {
        self
    }
}

impl DependOnRejectAuthorizeTokenService for Handler {
    type RejectAuthorizeTokenService = Self;
    fn reject_authorize_token_service(&self) -> &Self::RejectAuthorizeTokenService {
        self
    }
}

#[cfg(debug_assertions)]
mod mock {
    use axum::async_trait;
    use kernel::interfaces::transport::VerificationMailTransporter;
    use kernel::prelude::entities::{Address, MFACode};
    use kernel::KernelError;

    #[derive(Clone)]
    pub struct MockVerificationMailer;

    #[allow(clippy::new_without_default)]
    impl MockVerificationMailer {
        pub fn new() -> Self {
            Self
        }
    }

    #[async_trait]
    impl VerificationMailTransporter for MockVerificationMailer {
        async fn send(&self, address: &Address, code: &MFACode) -> Result<(), KernelError> {
            println!("code: {:?}, adr: {:?}", code, address);
            Ok(())
        }
    }
}
