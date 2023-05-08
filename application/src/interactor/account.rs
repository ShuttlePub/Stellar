use kernel::{
    repository::{AccountRepository, NonVerifiedAccountRepository}, 
    entities::{
        TicketId, 
        VerificationCode, 
        Account, 
        UserId, 
        NonVerifiedAccount, 
        Address, 
        UserName, 
        Password,
        UpdatedAt
    }, 
    transporter::VerificationMailTransporter, 
    KernelError
};
use kernel::external::{OffsetDateTime, Uuid};

use crate::{
    ApplicationError,
    adapter::account::{
        CreateAccountAdapter,
        CreateNonVerifiedAccountAdapter,
        UpdateAccountAdapter,
        DeleteAccountAdapter,
        ApproveAccountAdapter
    },
    transfer::account::{
        CreateAccountDto, 
        AccountDto, 
        CreateNonVerifiedAccountDto, 
        NonVerifiedAccountDto, UpdateAccountDto
    }
};

#[derive(Clone)]
pub struct CreateNonVerifiedAccountInteractor<T1, T2> {
    kvs: T1,

    #[cfg(not(debug_assertions))]
    mail: T2,
    
    #[cfg(debug_assertions)]
    _disabled: std::marker::PhantomData<T2>
}

impl<T1, T2> CreateNonVerifiedAccountInteractor<T1, T2> {
    #[cfg(not(debug_assertions))]
    pub fn new(kvs: T1, mail: T2) -> Self {
        Self { kvs, mail }
    }

    #[cfg(debug_assertions)]
    pub fn new(kvs: T1) -> Self {
        Self { kvs, _disabled: std::marker::PhantomData }
    }
}

#[async_trait::async_trait]
impl<T1, T2> CreateNonVerifiedAccountAdapter for CreateNonVerifiedAccountInteractor<T1, T2>
  where T1: NonVerifiedAccountRepository,
        T2: VerificationMailTransporter
{
    async fn create(&self, create: CreateNonVerifiedAccountDto) -> Result<NonVerifiedAccountDto, ApplicationError> {
        let ticket = TicketId::default();
        let code = VerificationCode::default();
        let CreateNonVerifiedAccountDto { address } = create;
        let non_verified = NonVerifiedAccount::new(ticket, address, code);
        
        self.kvs.create(&non_verified).await?;

        #[cfg(not(debug_assertions))]
        self.mail.send(non_verified.code(), non_verified.address()).await?;

        #[cfg(debug_assertions)]
        println!("{:?}", non_verified.code());

        Ok(non_verified.into())
    }
}

#[derive(Clone)]
pub struct ApproveAccountInteractor<T> {
    kvs: T
}

impl<T> ApproveAccountInteractor<T> {
    pub fn new(kvs: T) -> Self {
        Self { kvs }
    }
}

#[async_trait::async_trait]
impl<T> ApproveAccountAdapter for ApproveAccountInteractor<T>
  where T: NonVerifiedAccountRepository
{
    async fn approve(&self, id: &str, code: &str) -> Result<String, ApplicationError> {
        let id = TicketId::new(id);
        let code = VerificationCode::new(code);

        let Some(nonverified) = self.kvs.find_by_id(&id).await? else {
            return Err(ApplicationError::NotFound { 
                method: "find",
                entity: "non-verified account", 
                id: id.into()
            })
        };

        if !nonverified.is_match_verification_code(&code) {
            return Err(ApplicationError::InvalidValue { 
                method: "2FA code verify", 
                value: "verification code".into()
            });
        };

        let valid = TicketId::default();

        self.kvs.validation(&id, &valid, nonverified.address()).await?;

        Ok(valid.into())
    }
}

#[derive(Clone)]
pub struct CreateAccountInteractor<T1, T2> {
    repo: T1,
    kvs: T2
}

impl<T1, T2> CreateAccountInteractor<T1, T2> {
    pub fn new(repo: T1, kvs: T2) -> Self {
        Self { repo, kvs }
    }
}

#[async_trait::async_trait]
impl<T1, T2> CreateAccountAdapter for CreateAccountInteractor<T1, T2>
  where T1: AccountRepository,
        T2: NonVerifiedAccountRepository
{
    async fn create(&self, id: &str, create: CreateAccountDto) -> Result<AccountDto, ApplicationError> {
        let id = TicketId::new(id);

        let Some(verified) = self.kvs.find_by_valid_id(&id).await? else {
            return Err(ApplicationError::NotFound { 
                method: "find",
                entity: "temporary account", 
                id: id.into()
            })
        };

        let CreateAccountDto { name, pass } = create;

        let verified = Account::new(
            UserId::default(),
            verified,
            name,
            pass,
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc()
        )?;

        self.repo.create(&verified).await?;

        Ok(verified.into())
    }
}

#[derive(Clone)]
pub struct UpdateAccountInteractor<T> {
    repo: T
}

impl<T> UpdateAccountInteractor<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<T> UpdateAccountAdapter for UpdateAccountInteractor<T>
  where T: AccountRepository
{
    async fn update(&self, update: UpdateAccountDto) -> Result<AccountDto, ApplicationError> {
        let UpdateAccountDto { id, address, name, pass } = update;
        let id = UserId::new(id);
        let Some(account) = self.repo.find_by_id(&id).await? else {
            return Err(ApplicationError::NotFound { 
                method: "update",
                entity: "Account",
                id: id.as_ref().to_string() 
            })
        };

        account.pass().verify(&pass)
            .map_err(|e| match e {
                KernelError::Cryption(e) => ApplicationError::External(anyhow::Error::new(e)),
                KernelError::InvalidPassword(_) => ApplicationError::Verification { 
                    method: "on update in verification",
                    entity: "account",
                    id: id.as_ref().to_string()
                },
                _ => unreachable!()
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

        self.repo.update(&account).await?;

        Ok(account.into())
    }
}

#[derive(Clone)]
pub struct DeleteAccountInteractor<T> {
    repo: T
}

impl<T> DeleteAccountInteractor<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<T> DeleteAccountAdapter for DeleteAccountInteractor<T>
  where T: AccountRepository
{
    async fn delete(&self, pass: &str, delete: &Uuid) -> Result<(), ApplicationError> {
        let id = UserId::new(*delete);
        let Some(account) = self.repo.find_by_id(&id).await? else {
            return Err(ApplicationError::NotFound { 
                method: "delete",
                entity: "account",
                id: id.as_ref().to_string() 
            })
        };

        account.pass().verify(pass)
            .map_err(|e| match e {
                KernelError::Cryption(e) => ApplicationError::External(anyhow::Error::new(e)),
                KernelError::InvalidPassword(_) => ApplicationError::Verification { 
                    method: "on delete in verification",
                    entity: "account",
                    id: id.as_ref().to_string()
                },
                _ => unreachable!()
            })?;

        self.repo.delete(&id).await?;

        Ok(())
    }
}