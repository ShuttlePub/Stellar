use kernel::{
    repository::{
        ClientRegistry, 
        AccountRepository
    }, 
    entities::{
        Client, 
        ClientId, 
        ClientName, 
        ClientTypes, 
        ClientDescription, 
        RedirectUri, 
        Scopes, 
        Method, 
        MethodDescription, 
        UserId, 
        DestructAccount
    }
};
use uuid::Uuid;

use crate::{
    adaptor::client::RegisterClientAdaptor, 
    transfer::client::{
        ClientDto, 
        RegisterClientDto
    }, 
    ApplicationError
};

#[derive(Clone)]
pub struct RegisterClientInteractor<T1, T2> {
    registry: T1,
    accounts: T2
}

impl<T1, T2> RegisterClientInteractor<T1, T2> {
    pub fn new(registry: T1, accounts: T2) -> Self {
        Self { registry, accounts }
    }
}

#[async_trait::async_trait]
impl<T1, T2> RegisterClientAdaptor for RegisterClientInteractor<T1, T2>
  where T1: ClientRegistry,
        T2: AccountRepository
{
    async fn register(&self, register: RegisterClientDto) -> Result<ClientDto, ApplicationError> {
        let RegisterClientDto { 
            name, 
            desc, 
            uris, 
            owner,
            secret, 
            scopes 
        } = register;

        let owner = UserId::new(owner);

        let Some(owner) = self.accounts.find_by_id(&owner).await? else {
            return Err(ApplicationError::NotFound { 
                method: "account registration", 
                entity: "client", 
                id: format!("{:?}", owner)
            });
        };

        let DestructAccount { id, ..} = owner.into_destruct();
        let owner = id;
        let id = ClientId::new(Uuid::new_v4());
        let name = ClientName::new(name);
        let desc = ClientDescription::new(desc);
        let uris = uris.into_iter()
            .map(|uri| RedirectUri::new(uri))
            .collect::<Vec<_>>();
        let types = ClientTypes::new(secret);

        let mut scoped = Scopes::default();
        scopes.into_iter()
            .for_each(|scoping| 
                scoped.add((
                    Method::new(name.as_ref(), scoping.method), 
                    MethodDescription::new(scoping.description)
                ))
            );

        let client = Client::new(
            id,
            name,
            desc,
            uris,
            owner,
            types,
            scoped
        );

        self.registry.register(&client).await?;

        Ok(client.into())
    }
}