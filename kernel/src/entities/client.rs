use destructure::Destructure;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::account::{Address, UserId};
use crate::KernelError;


mod auth_method;
mod client_desc;
mod client_id;
mod client_name;
mod client_secret;
mod client_types;
mod client_uri;
mod contacts;
mod grant_type;
mod keys;
mod logo_uri;
mod policy_uri;
mod redirect;
mod regi_access_token;
mod regi_endpoint;
mod response_type;
mod scope;
mod tos_uri;

pub use self::{
    auth_method::*,
    client_desc::*,
    client_id::*,
    client_name::*,
    client_secret::*,
    client_types::*,
    client_uri::*,
    contacts::*,
    grant_type::*,
    keys::*,
    logo_uri::*,
    policy_uri::*,
    redirect::*,
    regi_access_token::*,
    regi_endpoint::*,
    response_type::*,
    scope::*,
    tos_uri::*
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
    redirect_uris: RedirectUris,
    scopes: Scopes,
    contact: Contacts,
    jwks: Option<Jwks>,
    conf_token: RegistrationAccessToken,
    conf_endpoint: RegistrationEndPoint,
}

impl Client {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<ClientId>,
        name: impl Into<String>,
        uri: impl AsRef<str>,
        desc: impl Into<String>,
        types: impl Into<ClientTypes>,
        logo: impl AsRef<str>,
        terms: impl AsRef<str>,
        owner: impl Into<Uuid>,
        policy: impl AsRef<str>,
        auth_method: impl Into<TokenEndPointAuthMethod>,
        grant_types: impl Into<Vec<GrantType>>,
        response_types: impl Into<Vec<ResponseType>>,
        redirect_uris: impl Into<Vec<RedirectUri>>,
        scopes: impl Into<Vec<(ScopeMethod, ScopeDescription)>>,
        contacts: impl Into<Vec<Address>>,
        jwk: impl Into<Option<String>>,
        conf_access_token: impl Into<String>,
        conf_endpoint: impl Into<String>,
    ) -> Result<Self, KernelError> {
        Ok(Self {
            id: id.into(),
            name: ClientName::new(name),
            uri: ClientUri::new(uri)?,
            desc: ClientDescription::new(desc),
            types: types.into(),
            logo: LogoUri::new(logo)?,
            terms: TermsUri::new(terms)?,
            owner: UserId::new(owner),
            policy: PolicyUri::new(policy)?,
            auth_method: auth_method.into(),
            grant_types: GrantTypes::new(grant_types),
            response_types: ResponseTypes::new(response_types),
            redirect_uris: RedirectUris::new(redirect_uris),
            scopes: Scopes::new(scopes),
            contact: Contacts::new(contacts.into()
                .into_iter()
                .collect::<Vec<_>>()
            ),
            jwks: jwk.into()
                .map(Jwks::new)
                .transpose()?,
            conf_token: RegistrationAccessToken::new(conf_access_token),
            conf_endpoint: RegistrationEndPoint::new(conf_endpoint)
        })
    }
}

impl Client {
    pub fn id(&self) -> &ClientId {
        &self.id
    }

    pub fn name(&self) -> &ClientName {
        &self.name
    }

    pub fn owner(&self) -> &UserId {
        &self.owner
    }

    pub fn client_uri(&self) -> &ClientUri {
        &self.uri
    }

    pub fn description(&self) -> &ClientDescription {
        &self.desc
    }

    pub fn logo_uri(&self) -> &LogoUri {
        &self.logo
    }

    pub fn contacts(&self) -> &Contacts {
        &self.contact
    }

    pub fn tos_uri(&self) -> &TermsUri {
        &self.terms
    }

    pub fn policy_uri(&self) -> &PolicyUri {
        &self.policy
    }

    pub fn types(&self) -> &ClientTypes {
        &self.types
    }

    pub fn auth_method(&self) -> &TokenEndPointAuthMethod {
        &self.auth_method
    }

    pub fn grant_types(&self) -> &GrantTypes {
        &self.grant_types
    }

    pub fn response_types(&self) -> &ResponseTypes {
        &self.response_types
    }

    pub fn jwks(&self) -> &Option<Jwks> {
        &self.jwks
    }

    pub fn redirect_uris(&self) -> &RedirectUris {
        &self.redirect_uris
    }

    pub fn scopes(&self) -> &Scopes {
        &self.scopes
    }

    pub fn conf_token(&self) -> &RegistrationAccessToken {
        &self.conf_token
    }

    pub fn conf_endpoint(&self) -> &RegistrationEndPoint {
        &self.conf_endpoint
    }
}