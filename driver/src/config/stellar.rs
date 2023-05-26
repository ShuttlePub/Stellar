use kernel::{
    entities::{
        TokenEndPointAuthMethod,
        TermsUri,
        Scopes,
        ScopeMethod,
        ScopeDescription,
        ResponseTypes,
        ResponseType,
        RegistrationEndPoint,
        RegistrationAccessToken,
        RedirectUris,
        RedirectUri,
        PolicyUri,
        LogoUri,
        Jwks,
        GrantTypes,
        GrantType,
        Contacts,
        ClientUri,
        ClientTypes,
        ClientSecret,
        ClientName,
        ClientId,
        ClientDescription,
        Address,
        UserId
    }
};
use kernel::entities::Client;
use kernel::services::JwkSelectionService;
use super::Stellar;
use crate::DriverError;

#[derive(Debug)]
pub struct StellarClient {
    client_id: ClientId,
    name: ClientName,
    client_uri: ClientUri,
    client_desc: ClientDescription,
    types: ClientTypes,
    logo_uri: LogoUri,
    tos_uri: TermsUri,
    owner: UserId,
    policy_uri: PolicyUri,
    auth_method: TokenEndPointAuthMethod,
    grant_types: GrantTypes,
    response_types: ResponseTypes,
    redirect_uris: RedirectUris,
    scopes: Scopes,
    contacts: Contacts,
    jwk: Jwks,
    conf_access_token: RegistrationAccessToken,
    conf_endpoint: RegistrationEndPoint,
}

impl StellarClient {
    pub fn client_id(&mut self, client_id: ClientId) {
        self.client_id = client_id
    }

    pub fn owner(&mut self, admin_id: UserId) {
        self.owner = admin_id
    }
}

impl Default for StellarClient {
    fn default() -> Self {
        Self {
            client_id: ClientId::default(),
            name: ClientName::new("Stellar"),
            client_uri: ClientUri::new("https://stellar.example.com").unwrap(),
            client_desc: ClientDescription::new("OAuth2.0 Authentication Provider"),
            types: ClientTypes::Confidential(ClientSecret::default()),
            logo_uri: LogoUri::new("https://stellar.example.com/logo").unwrap(),
            tos_uri: TermsUri::new("https://stellar.example.com/terms").unwrap(),
            owner: UserId::default(),
            policy_uri: PolicyUri::new("https://stellar.example.com/policy").unwrap(),
            auth_method: TokenEndPointAuthMethod::PrivateKeyJWT,
            grant_types: GrantTypes::new(vec![GrantType::AuthorizationCode]),
            response_types: ResponseTypes::new(vec![ResponseType::Code]),
            redirect_uris: RedirectUris::new(vec![RedirectUri::new("https://stellar.example.com/callback")]),
            scopes: Scopes::new(vec![
                ("read", "read user data from stellar"),
                ("write", "write additional data into user data")
            ].into_iter()
             .map(|(m, d)| (ScopeMethod::new(m), ScopeDescription::new(d.to_string())))
             .collect::<Vec<_>>()),
            contacts: Contacts::new(vec![Address::new("admin.example@stellar.example.com")]),
            jwk: Jwks::new("https://stellar.example.com/.well-known").unwrap(),
            conf_access_token: RegistrationAccessToken::default(),
            conf_endpoint: RegistrationEndPoint::default(),
        }
    }
}

impl TryFrom<Stellar> for StellarClient {
    type Error = DriverError;
    fn try_from(value: Stellar) -> Result<Self, Self::Error> {
        Ok(Self {
            client_uri: ClientUri::new(value.client_uri)?,
            logo_uri: LogoUri::new(value.logo_uri)?,
            tos_uri: TermsUri::new(value.tos_uri)?,
            policy_uri: PolicyUri::new(value.policy_uri)?,
            contacts: Contacts::new(value.contacts.into_iter().map(Address::new).collect::<Vec<_>>()),
            jwk: JwkSelectionService::check(value.jwks, value.jwks_uri)?.unwrap(),
            ..Default::default()
        })
    }
}

impl TryFrom<StellarClient> for Client {
    type Error = DriverError;
    fn try_from(value: StellarClient) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.client_id, 
            value.name, 
            value.client_uri, 
            value.client_desc, 
            value.types, 
            value.logo_uri, 
            value.tos_uri, 
            value.owner, 
            value.policy_uri, 
            value.auth_method, 
            value.grant_types, 
            value.response_types, 
            value.redirect_uris, 
            value.scopes, 
            value.contacts, 
            value.jwk, 
            value.conf_access_token, 
            value.conf_endpoint)?)
    }
}