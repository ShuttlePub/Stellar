use kernel::external::{Duration, OffsetDateTime, Uuid};
use kernel::interfaces::repository::{
    AcceptedActionVolatileRepository, AccountRepository, DependOnAcceptedActionVolatileRepository,
    DependOnAccountRepository, DependOnMFACodeVolatileRepository,
    DependOnPendingActionVolatileRepository, DependOnSessionVolatileRepository,
    DependOnTemporaryAccountRepository, MFACodeVolatileRepository, PendingActionVolatileRepository,
    SessionVolatileRepository, TemporaryAccountRepository,
};
use kernel::prelude::entities::{
    Account, Address, EstablishedAt, MFACode, Password, Session, SessionId, TemporaryAccount,
    TicketId, UpdatedAt, UserId, UserName,
};
use kernel::KernelError;

#[allow(unused_imports)]
use kernel::interfaces::transport::{
    DependOnVerificationMailTransporter, VerificationMailTransporter,
};

use crate::services::DependOnVerifyMFACodeService;
use crate::transfer::mfa_code::TicketIdDto;
use crate::{
    transfer::{
        account::{
            AccountDto, CreateAccountDto, CreateTemporaryAccountDto, UpdateAccountDto,
            VerifyAccountDto,
        },
        session::SessionDto,
    },
    ApplicationError, ExpectUserAction,
};

#[async_trait::async_trait]
pub trait CreateTemporaryAccountService:
    'static
    + Send
    + Sync
    + DependOnTemporaryAccountRepository
    + DependOnVerificationMailTransporter
    + DependOnVerifyMFACodeService
{
    async fn create(
        &self,
        create: CreateTemporaryAccountDto,
    ) -> Result<TicketIdDto, ApplicationError> {
        let user_id = UserId::default();
        let ticket = TicketId::default();
        let code = MFACode::default();
        let CreateTemporaryAccountDto { address } = create;
        let non_verified = TemporaryAccount::new(user_id, address);

        self.temporary_account_repository()
            .create(&non_verified)
            .await?;
        self.verify_mfa_code_service()
            .pending_action_volatile_repository()
            .create(&ticket, &user_id)
            .await?;
        self.verify_mfa_code_service()
            .mfa_code_volatile_repository()
            .create(&user_id, &code)
            .await?;
        self.verification_mail_transporter()
            .send(non_verified.address(), &code)
            .await?;

        Ok(ticket.into())
    }
}

pub trait DependOnCreateNonVerifiedAccountService: 'static + Send + Sync {
    type CreateNonVerifiedAccountService: CreateTemporaryAccountService;
    fn create_non_verified_account_service(&self) -> &Self::CreateNonVerifiedAccountService;
}

#[async_trait::async_trait]
pub trait CreateAccountService:
    'static
    + Send
    + Sync
    + DependOnAccountRepository
    + DependOnTemporaryAccountRepository
    + DependOnVerifyMFACodeService
{
    async fn create(
        &self,
        ticket: &str,
        create: CreateAccountDto,
    ) -> Result<AccountDto, ApplicationError> {
        let ticket = TicketId::new(ticket);

        let account = self
            .verify_mfa_code_service()
            .accepted_action_volatile_repository()
            .find(&ticket)
            .await?
            .ok_or_else(|| ApplicationError::NotFound {
                method: "find",
                entity: "ticket:accepted",
                id: ticket.into(),
            })?;

        let account = self
            .temporary_account_repository()
            .find_by_id(&account)
            .await?
            .ok_or_else(|| ApplicationError::NotFound {
                method: "find_by_id",
                entity: "account:temporary",
                id: account.to_string(),
            })?;

        let CreateAccountDto { name, pass } = create;

        let verified = Account::new(
            UserId::default(),
            account.address().as_ref(),
            name,
            pass,
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc(),
        )?;

        self.account_repository().create(&verified).await?;

        Ok(verified.into())
    }
}

pub trait DependOnCreateAccountService: 'static + Send + Sync {
    type CreateAccountService: CreateAccountService;
    fn create_account_service(&self) -> &Self::CreateAccountService;
}

#[async_trait::async_trait]
pub trait UpdateAccountService: 'static + Send + Sync + DependOnAccountRepository {
    async fn update(&self, update: UpdateAccountDto) -> Result<AccountDto, ApplicationError> {
        let UpdateAccountDto {
            id,
            address,
            name,
            pass,
        } = update;
        let id = UserId::new(id);
        let Some(account) = self.account_repository().find_by_id(&id).await? else {
            return Err(ApplicationError::NotFound {
                method: "update",
                entity: "Account",
                id: AsRef::<Uuid>::as_ref(&id).to_string(),
            });
        };

        account.pass().verify(&pass).map_err(|e| match e {
            KernelError::Cryption(e) => ApplicationError::External(anyhow::Error::new(e)),
            KernelError::InvalidPassword(_) => ApplicationError::Verification {
                method: "on update in verification",
                entity: "account",
                id: AsRef::<Uuid>::as_ref(&id).to_string(),
            },
            _ => unreachable!(),
        })?;

        let mut account = account.into_destruct();
        let mut date = account.date.into_destruct();

        account.address = Address::new(address);
        account.name = UserName::new(name);
        account.pass = Password::new(pass)?;

        date.updated_at = UpdatedAt::new(OffsetDateTime::now_utc());
        let date = date.freeze();

        account.date = date;

        let account = account.freeze();

        self.account_repository().update(&account).await?;

        Ok(account.into())
    }
}

pub trait DependOnUpdateAccountService: 'static + Send + Sync {
    type UpdateAccountService: UpdateAccountService;
    fn update_account_service(&self) -> &Self::UpdateAccountService;
}

#[async_trait::async_trait]
pub trait DeleteAccountService: 'static + Send + Sync + DependOnAccountRepository {
    async fn delete(&self, pass: &str, delete: &Uuid) -> Result<(), ApplicationError> {
        let id = UserId::new(*delete);
        let Some(account) = self.account_repository().find_by_id(&id).await? else {
            return Err(ApplicationError::NotFound {
                method: "delete",
                entity: "account",
                id: AsRef::<Uuid>::as_ref(&id).to_string(),
            });
        };

        account.pass().verify(pass).map_err(|e| match e {
            KernelError::Cryption(e) => ApplicationError::External(anyhow::Error::new(e)),
            KernelError::InvalidPassword(_) => ApplicationError::Verification {
                method: "on delete in verification",
                entity: "account",
                id: AsRef::<Uuid>::as_ref(&id).to_string(),
            },
            _ => unreachable!(),
        })?;

        self.account_repository().delete(&id).await?;

        Ok(())
    }
}

pub trait DependOnDeleteAccountService: 'static + Send + Sync {
    type DeleteAccountService: DeleteAccountService;
    fn delete_account_service(&self) -> &Self::DeleteAccountService;
}

#[async_trait::async_trait]
pub trait VerifyAccountService:
    'static
    + Send
    + Sync
    + DependOnAccountRepository
    + DependOnVerificationMailTransporter
    + DependOnSessionVolatileRepository
    + DependOnVerifyMFACodeService
{
    async fn verify(&self, verify: VerifyAccountDto) -> Result<SessionDto, ApplicationError> {
        let VerifyAccountDto {
            ticket,
            address,
            pass,
            session,
            ..
        } = verify;

        if let Some(session) = session {
            let session = SessionId::new(session);
            if let Some(valid) = self.session_volatile_repository().find(&session).await? {
                return if valid.exp().is_expired() {
                    self.session_volatile_repository()
                        .revoke(valid.id())
                        .await?;
                    Err(ApplicationError::RequireUserAction(ExpectUserAction::Login))
                } else {
                    let id = SessionId::default();
                    let usr = *valid.usr();
                    let exp = Duration::new(60 * 60, 0);
                    let est = EstablishedAt::default();
                    let regen = Session::new(id, usr, exp, est);
                    self.session_volatile_repository()
                        .revoke(valid.id())
                        .await?;
                    self.session_volatile_repository().establish(&regen).await?;
                    Ok(regen.into())
                };
            }
        }

        match ticket {
            None => {
                if address.is_none() || pass.is_none() {
                    return Err(ApplicationError::InvalidValue {
                        method: "required field valid",
                        value: "required field `address` and `pass`.".to_string(),
                    });
                }

                let address = Address::new(address.unwrap());

                let Some(account) = self.account_repository().find_by_address(&address).await?
                else {
                    return Err(ApplicationError::NotFound {
                        method: "find_by_address",
                        entity: "account",
                        id: address.into(),
                    });
                };

                account.pass().verify(pass.unwrap())?;

                let code = MFACode::default();
                self.verify_mfa_code_service()
                    .mfa_code_volatile_repository()
                    .create(account.id(), &code)
                    .await?;
                self.verification_mail_transporter()
                    .send(account.address(), &code)
                    .await?;

                Err(ApplicationError::RequireUserAction(ExpectUserAction::MFA))
            }
            Some(ticket) => {
                let ticket = TicketId::new(ticket);
                let usr = self
                    .verify_mfa_code_service()
                    .accepted_action_volatile_repository()
                    .find(&ticket)
                    .await?
                    .ok_or_else(|| ApplicationError::NotFound {
                        method: "find",
                        entity: "ticket:accepted",
                        id: ticket.into(),
                    })?;
                let id = SessionId::default();
                let exp = Duration::new(60 * 60, 0);
                let est = EstablishedAt::default();
                let session = Session::new(id, usr, exp, est);
                self.session_volatile_repository()
                    .establish(&session)
                    .await?;

                Ok(session.into())
            }
        }
    }
}

pub trait DependOnVerifyAccountService: 'static + Send + Sync {
    type VerifyAccountService: VerifyAccountService;
    fn verify_account_service(&self) -> &Self::VerifyAccountService;
}
