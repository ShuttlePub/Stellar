use crate::services::{
    CreateAccountService, CreateTemporaryAccountService, DeleteAccountService,
    DependOnVerifyMFACodeService, UpdateAccountService, VerifyAccountService,
};
use kernel::interfaces::repository::{
    DependOnAccountRepository, DependOnSessionVolatileRepository,
    DependOnTemporaryAccountRepository,
};
use kernel::interfaces::transport::DependOnVerificationMailTransporter;

// Default Impl
impl<T> CreateTemporaryAccountService for T where
    T: DependOnTemporaryAccountRepository
        + DependOnVerificationMailTransporter
        + DependOnVerifyMFACodeService
{
}

// Default Impl
impl<T> CreateAccountService for T where
    T: DependOnAccountRepository
        + DependOnTemporaryAccountRepository
        + DependOnVerifyMFACodeService
{
}

// Default Impl
impl<T> UpdateAccountService for T where T: DependOnAccountRepository {}

// Default Impl
impl<T> DeleteAccountService for T where T: DependOnAccountRepository {}

// Default Impl
impl<T> VerifyAccountService for T where
    T: DependOnAccountRepository
        + DependOnSessionVolatileRepository
        + DependOnVerificationMailTransporter
        + DependOnVerifyMFACodeService
{
}
