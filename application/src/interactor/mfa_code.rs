use kernel::repository::{DependOnAcceptedActionVolatileRepository, DependOnAccountRepository, DependOnMFACodeVolatileRepository, DependOnPendingActionVolatileRepository};
use crate::services::VerifyMFACodeService;

impl<T> VerifyMFACodeService for T
    where T: DependOnAccountRepository
           + DependOnMFACodeVolatileRepository
           + DependOnPendingActionVolatileRepository
           + DependOnAcceptedActionVolatileRepository {}