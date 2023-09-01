use crate::services::VerifyMFACodeService;
use kernel::interfaces::repository::{
    DependOnAcceptedActionVolatileRepository, DependOnAccountRepository,
    DependOnMFACodeVolatileRepository, DependOnPendingActionVolatileRepository,
};

impl<T> VerifyMFACodeService for T where
    T: DependOnAccountRepository
        + DependOnMFACodeVolatileRepository
        + DependOnPendingActionVolatileRepository
        + DependOnAcceptedActionVolatileRepository
{
}
