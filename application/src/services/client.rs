use kernel::prelude::entities::{ClientId, ClientTypes};
use kernel::external::Uuid;
use kernel::interfaces::repository::{AccountRepository, ClientRegistry, DependOnAccountRepository, DependOnClientRegistry};
use crate::ApplicationError;
use crate::transfer::client::{ClientDto, RegisterClientDto, UpdateClientDto};

#[async_trait::async_trait]
pub trait RegisterClientService: 'static + Sync + Send
    + DependOnClientRegistry
    + DependOnAccountRepository
{
    async fn register(&self, register: RegisterClientDto) -> Result<ClientDto, ApplicationError>;
}

pub trait DependOnRegisterClientService: 'static + Sync + Send {
    type RegisterClientService: RegisterClientService;
    fn register_client_service(&self) -> &Self::RegisterClientService;
}

#[async_trait::async_trait]
pub trait UpdateClientService: 'static + Sync + Send
    + DependOnClientRegistry
    + DependOnAccountRepository
{
    async fn update(&self, id: &Uuid, cl_secret: &str, pass_phrase: &str, update: UpdateClientDto) -> Result<ClientDto, ApplicationError>;
}

pub trait DependOnUpdateClientService: 'static + Sync + Send {
    type UpdateClientService: UpdateClientService;
    fn update_client_service(&self) -> &Self::UpdateClientService;
}

#[async_trait::async_trait]
pub trait DeleteClientService: 'static + Sync + Send
    + DependOnClientRegistry
    + DependOnAccountRepository
{
    //noinspection DuplicatedCode
    async fn delete(&self, id: &Uuid, cl_secret: &str, pass_phrase: &str) -> Result<(), ApplicationError> {
        let client_id = ClientId::new_at_now(*id);

        let Some(client) = self.client_registry().find_by_id(&client_id).await? else {
            return Err(ApplicationError::NotFound {
                method: "find_by_id",
                entity: "client",
                id: client_id.to_string(),
            })
        };

        if let ClientTypes::Confidential(secret) = client.types() {
            if let Err(e) = secret.verify(cl_secret) {
                return Err(ApplicationError::Verification {
                    method: "client_secret_verify",
                    entity: "client",
                    id: format!("{:?}, `in kernel`: {:?}", client_id, e),
                })
            }
        }

        let Some(owner_ac) = self.account_repository().find_by_id(client.owner()).await? else {
            return Err(ApplicationError::NotFound {
                method: "find_by_id",
                entity: "account",
                id: client.owner().to_string(),
            })
        };

        if let Err(e) = owner_ac.pass().verify(pass_phrase) {
            return Err(ApplicationError::Verification {
                method: "account_password_verify",
                entity: "account",
                id: format!("{:?}, `in kernel`: {:?}", owner_ac.id(), e),
            })
        }

        self.client_registry().delete(&client_id).await?;

        Ok(()) 
    }
}

pub trait DependOnDeleteClientService: 'static + Sync + Send {
    type DeleteClientService: DeleteClientService;
    fn delete_client_service(&self) -> &Self::DeleteClientService;
}