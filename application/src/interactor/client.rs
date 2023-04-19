use kernel::{repository::ClientRegistry, entities::{Client, ClientId, ClientName, ClientDescription, RedirectUri, ClientTypes, Scopes, Method, MethodDescription}};
use uuid::Uuid;

use crate::{adaptor::client::RegisterClientAdaptor, transfer::client::{ClientDto, RegisterClientDto}, ApplicationError};

#[derive(Clone)]
pub struct RegisterClientInteractor<T> {
    registry: T
}

impl<T> RegisterClientInteractor<T> {
    pub fn new(registry: T) -> Self {
        Self { registry }
    }
}

#[async_trait::async_trait]
impl<T> RegisterClientAdaptor for RegisterClientInteractor<T>
  where T: ClientRegistry
{
    async fn register(&self, register: RegisterClientDto) -> Result<ClientDto, ApplicationError> {
        let RegisterClientDto { 
            name, 
            desc, 
            uris, 
            secret, 
            scopes 
        } = register;

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
            types,
            scoped
        );

        self.registry.register(&client).await?;

        Ok(client.into())
    }
}