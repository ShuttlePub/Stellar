use destructure::Destructure;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::entities::{Address, GrantType, ResponseType, ScopeDescription, ScopeMethod};
use crate::KernelError;

use super::{
    ClientId,
    ClientName,
    ClientTypes,
    ClientUri,
    ClientDescription,
    Scopes,
    UserId,
    GrantTypes,
    ResponseTypes,
    TokenEndPointAuthMethod,
    LogoUri,
    TermsUri,
    Contacts,
    Jwks,
    PolicyUri
};

/// Client.
///
/// Reference:
/// [RFC6749](https://www.rfc-editor.org/rfc/rfc6749#section-2)
/// [RFC7591](https://www.rfc-editor.org/rfc/rfc7591#section-2)
#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct Client {
    id: ClientId,
    name: ClientName,
    uri: ClientUri,
    desc: ClientDescription,
    types: ClientTypes,
    logo: LogoUri,
    terms: TermsUri,
    owner: UserId,
    policy: PolicyUri,
    auth_method: TokenEndPointAuthMethod,
    grant_types: GrantTypes,
    response_types: ResponseTypes,
    scopes: Scopes,
    contact: Contacts,
    jwks: Option<Jwks>
}

impl Client {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<Uuid>,
        name: impl Into<String>,
        uri: impl AsRef<str>,
        desc: impl Into<String>,
        secret: impl Into<Option<String>>,
        logo: impl AsRef<str>,
        terms: impl AsRef<str>,
        owner: impl Into<Uuid>,
        policy: impl AsRef<str>,
        auth_method: impl Into<TokenEndPointAuthMethod>,
        grant_types: impl Into<Vec<GrantType>>,
        response_types: impl Into<Vec<ResponseType>>,
        scopes: impl Into<Vec<(ScopeMethod, ScopeDescription)>>,
        contacts: impl Into<Vec<String>>,
        jwk: impl Into<Option<String>>
    ) -> Result<Self, KernelError> {
        Ok(Self {
            id: ClientId::new_at_now(id),
            name: ClientName::new(name),
            uri: ClientUri::new(uri)?,
            desc: ClientDescription::new(desc),
            types: ClientTypes::new(secret),
            logo: LogoUri::new(logo)?,
            terms: TermsUri::new(terms)?,
            owner: UserId::new(owner),
            policy: PolicyUri::new(policy)?,
            auth_method: auth_method.into(),
            grant_types: GrantTypes::new(grant_types),
            response_types: ResponseTypes::new(response_types),
            scopes: Scopes::new(scopes),
            contact: Contacts::new(contacts.into()
                .into_iter()
                .map(Address::new)
                .collect::<Vec<_>>()
            ),
            jwks: jwk.into()
                .map(Jwks::new)
                .transpose()?
        })
    }
}