use kernel::prelude::entities::{MFACode, TicketId};
use kernel::interfaces::repository::{AcceptedActionVolatileRepository, DependOnAcceptedActionVolatileRepository, DependOnAccountRepository, DependOnMFACodeVolatileRepository, DependOnPendingActionVolatileRepository, MFACodeVolatileRepository, PendingActionVolatileRepository};
use crate::ApplicationError;
use crate::transfer::mfa_code::{MFAActionDto, TicketIdDto};

#[async_trait::async_trait]
pub trait VerifyMFACodeService: 'static + Sync + Send
    + DependOnAccountRepository
    + DependOnMFACodeVolatileRepository
    + DependOnPendingActionVolatileRepository
    + DependOnAcceptedActionVolatileRepository
{
    async fn verify(&self, action: MFAActionDto) -> Result<TicketIdDto, ApplicationError> {
        let pending = TicketId::new(action.pending);
        let Some(account) = self.pending_action_volatile_repository().find(&pending).await? else {
            return Err(ApplicationError::NotFound {
                method: "find",
                entity: "ticket#pending",
                id: pending.into(),
            })
        };

        let Some(origin) = self.mfa_code_volatile_repository().find_by_id(&account).await? else {
            return Err(ApplicationError::NotFound {
                method: "find",
                entity: "mfa_code",
                id: account.to_string(),
            })
        };

        let code = MFACode::new(action.code);

        if code.ne(&origin) {
            return Err(ApplicationError::InvalidValue {
                method: "MFACode equivalence comparison",
                value: code.into(),
            })
        }

        self.pending_action_volatile_repository().revoke(&pending).await?;
        self.mfa_code_volatile_repository().revoke(&account).await?;

        let verified = TicketId::default();

        self.accepted_action_volatile_repository().create(&verified, &account).await?;

        Ok(verified.into())
    }
}


pub trait DependOnVerifyMFACodeService: 'static + Sync + Send {
    type VerifyMFACodeService: VerifyMFACodeService;
    fn verify_mfa_code_service(&self) -> &Self::VerifyMFACodeService;
}