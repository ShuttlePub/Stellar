use destructure::Destructure;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::{ClientId, ClientName, ClientDescription, RedirectUri, Scopes, ClientTypes};

/// Client information based on RFC6749.
/// 
/// See [RFC6749 Section 2](https://www.rfc-editor.org/rfc/rfc6749#section-2)
#[derive(Debug, Clone, Hash, Deserialize, Serialize, Destructure)]
pub struct Client {
    /// An identifier to identify the client.
    /// 
    /// Reference [RFC6749 Section 2.2](https://www.rfc-editor.org/rfc/rfc6749#section-2.2)
    id: ClientId,
    /// An name to display the client
    name: ClientName,
    /// An description to display the client
    desc: ClientDescription,
    /// It is an absolute URI, 
    /// that user will be redirected to 
    /// when user finish the authorized server exchange.
    /// 
    /// Reference [RFC6749 Section 3.1.2](https://www.rfc-editor.org/rfc/rfc6749#section-3.1.2)
    uris: Option<Vec<RedirectUri>>,
    /// A client type, segregated by the client's ability to maintain confidentiality.
    /// 
    /// Reference [RFC6749 Section 2.1](https://www.rfc-editor.org/rfc/rfc6749#section-2.1)
    types: ClientTypes,
    /// A set of scopes allowed to users defined by the client.
    /// 
    /// Reference [RFC6749 Section 3.3](https://www.rfc-editor.org/rfc/rfc6749#section-3.3)
    scopes: Scopes,
}

impl Client {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<Uuid>,
        name: impl Into<String>,
        desc: impl Into<String>,
        uris: impl Into<Option<Vec<RedirectUri>>>,
        secret: impl Into<Option<String>>,
        scopes: impl Into<Scopes>,
    ) -> Self {
        Self { 
            id: ClientId::new(id), 
            name: ClientName::new(name),
            desc: ClientDescription::new(desc), 
            uris: uris.into(), 
            types: ClientTypes::new(secret), 
            scopes: scopes.into()
        }
    }

    pub fn id(&self) -> &ClientId {
        &self.id
    }

    pub fn name(&self) -> &ClientName {
        &self.name
    }

    pub fn description(&self) -> &ClientDescription {
        &self.desc
    }

    pub fn uris(&self) -> &Option<Vec<RedirectUri>> {
        &self.uris
    }

    pub fn types(&self) -> &ClientTypes {
        &self.types
    }

    pub fn scopes(&self) -> &Scopes {
        &self.scopes
    }
}