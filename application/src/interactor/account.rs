use kernel::repository::{DependOnAccountRepository, DependOnMFACodeVolatileRepository, DependOnNonVerifiedAccountRepository, DependOnSessionVolatileRepository};
use kernel::transport::DependOnVerificationMailTransporter;
use crate::services::{CreateNonVerifiedAccountService, ApproveAccountService, UpdateAccountService, CreateAccountService, DeleteAccountService, VerifyAccountService};

// Default Impl
impl<T> CreateNonVerifiedAccountService for T
    where T: DependOnNonVerifiedAccountRepository
           + DependOnVerificationMailTransporter {}


// Default Impl
impl<T> ApproveAccountService for T
    where T: DependOnNonVerifiedAccountRepository {}


// Default Impl
impl<T> CreateAccountService for T
    where T: DependOnAccountRepository
    + DependOnNonVerifiedAccountRepository {}


// Default Impl
impl<T> UpdateAccountService for T
    where T: DependOnAccountRepository {}


// Default Impl
impl<T> DeleteAccountService for T
    where T: DependOnAccountRepository {}

impl<T> VerifyAccountService for T
    where T: DependOnAccountRepository
           + DependOnSessionVolatileRepository
           + DependOnMFACodeVolatileRepository
           + DependOnVerificationMailTransporter {}