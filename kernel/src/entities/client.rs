use destructure::Destructure;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::{ClientId, ClientName, RedirectUri, Scopes, ClientTypes};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct ClientDescription(String);

impl ClientDescription {
    pub fn new(description: impl Into<String>) -> Self {
        Self(description.into())
    }
}

impl From<ClientDescription> for String {
    fn from(origin: ClientDescription) -> Self {
        origin.0
    }
}

impl AsRef<str> for ClientDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Hash, Deserialize, Serialize, Destructure)]
pub struct Client {
    id: ClientId,
    name: ClientName,
    types: ClientTypes,
    description: ClientDescription,
    redirect_uri: RedirectUri,
    support_scope: Scopes
}

impl Client {
    pub fn new(
        id: impl Into<Uuid>,
        name: impl Into<String>,
        secret: impl Into<Option<String>>,
        description: impl Into<String>,
        redirect_uri: impl Into<String>,
        support_scope: impl Into<Vec<String>>
    ) -> Self {
        Self { 
            id: ClientId::new(id), 
            name: ClientName::new(name), 
            types: ClientTypes::new(secret),
            description: ClientDescription::new(description),
            redirect_uri: RedirectUri::new(redirect_uri),
            support_scope: Scopes::new(support_scope.into())
        }
    }

    pub fn id(&self) -> &ClientId {
        &self.id
    }

    pub fn name(&self) -> &ClientName {
        &self.name
    }

    pub fn types(&self) -> &ClientTypes {
        &self.types
    }

    pub fn description(&self) -> &ClientDescription {
        &self.description
    }

    pub fn redirect_uri(&self) -> &RedirectUri {
        &self.redirect_uri
    }

    pub fn support_scope(&self) -> &Scopes {
        &self.support_scope
    }
}