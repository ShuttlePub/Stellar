use kernel::interfaces::repository::{DependOnAccountRepository, DependOnTemporaryAccountRepository, DependOnSessionVolatileRepository};
use kernel::interfaces::transport::DependOnVerificationMailTransporter;
use crate::services::{
    CreateTemporaryAccountService,
    UpdateAccountService,
    CreateAccountService,
    DeleteAccountService,
    VerifyAccountService,
    DependOnVerifyMFACodeService
};

// Default Impl
impl<T> CreateTemporaryAccountService for T
    where T: DependOnTemporaryAccountRepository
           + DependOnVerificationMailTransporter
           + DependOnVerifyMFACodeService {}


// Default Impl
impl<T> CreateAccountService for T
    where T: DependOnAccountRepository
           + DependOnTemporaryAccountRepository
           + DependOnVerifyMFACodeService {}


// Default Impl
impl<T> UpdateAccountService for T
    where T: DependOnAccountRepository {}


// Default Impl
impl<T> DeleteAccountService for T
    where T: DependOnAccountRepository {}


// Default Impl
impl<T> VerifyAccountService for T
    where T: DependOnAccountRepository
           + DependOnSessionVolatileRepository
           + DependOnVerificationMailTransporter
           + DependOnVerifyMFACodeService {}