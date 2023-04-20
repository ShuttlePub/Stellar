use kernel::entities::{Client, DestructClient, ScopedObject, DestructScopedObject};
use uuid::Uuid;

#[derive(Debug)]
pub struct ClientDto {
    pub id: Uuid,
    pub name: String,
    pub desc: String,
    pub uris: Option<Vec<String>>,
    pub owner: Uuid,
    pub types: Option<String>,
    pub scopes: Vec<ScopeDto>
}

#[derive(Debug)]
pub struct ScopeDto {
    pub method: String,
    pub description: String
}

impl From<Client> for ClientDto {
    fn from(value: Client) -> Self {
        let DestructClient {
            id,
            name,
            desc,
            uris,
            owner,
            types,
            scopes,
        } = value.into_destruct();
        Self { 
            id: id.into(), 
            name: name.into(), 
            desc: desc.into(), 
            uris: uris.map(|uris| 
                uris.into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
                ), 
            owner: owner.into(),
            types: types.into(), 
            scopes: Vec::from(scopes).into_iter()
                        .map(|obj: ScopedObject| ScopeDto::from(obj))
                        .collect()
        }
    }
}

impl From<ScopedObject> for ScopeDto {
    fn from(value: ScopedObject) -> Self {
        let DestructScopedObject { 
            method, 
            description 
        } = value.into_destruct();
        Self { 
            method: method.into(), 
            description: description.into() 
        }
    }
}

#[derive(Debug)]
pub struct RegisterClientDto {
    pub name: String,
    pub desc: String,
    pub uris: Vec<String>,
    pub owner: Uuid,
    pub secret: Option<String>,
    pub scopes: Vec<ScopeDto>
}