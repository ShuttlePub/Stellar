use kernel::{
    repository::{
        ClientRegistry, 
        AccountRepository
    }, 

};
use kernel::entities::{Address, Client, ClientDescription, ClientId, ClientName, ClientSecret, ClientTypes, ClientUri, Contacts, GrantType, GrantTypes, Jwks, LogoUri, PolicyUri, ResponseType, ResponseTypes, ScopeDescription, ScopeMethod, Scopes, TermsUri, TokenEndPointAuthMethod, UserId};

use crate::{
    adaptor::client::RegisterClientAdaptor, 
    transfer::client::{
        ClientDto, 
        RegisterClientDto,
        GrantTypeDto,
        ResponseTypeDto,
        TokenEndPointAuthMethodDto
    }, 
    ApplicationError,
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
            id,
            name,
            client_uri,
            description,
            logo_uri,
            tos_uri,
            owner_id,
            policy_uri,
            auth_method,
            grant_types,
            response_types,
            scopes,
            contacts,
            jwk
        } = register;

        let owner = UserId::new(owner_id);

        let Some(owner) = self.accounts.find_by_id(&owner).await? else {
            return Err(ApplicationError::NotFound {
                method: "find_by_id",
                entity: "account",
                id: owner.to_string(),
            })
        };

        let owner = owner.into_destruct();

        let types = match auth_method {
            TokenEndPointAuthMethodDto::ClientSecretPost |
            TokenEndPointAuthMethodDto::ClientSecretBasic |
            TokenEndPointAuthMethodDto::PrivateKeyJWK => ClientSecret::default().into(),
            TokenEndPointAuthMethodDto::None => ClientTypes::new(None)
        };

        let client_id = ClientId::new_at_now(id);
        let name = ClientName::new(name);
        let client_uri = ClientUri::new(client_uri)?;
        let client_desc = ClientDescription::new(description);
        let logo_uri = LogoUri::new(logo_uri)?;
        let tos_uri = TermsUri::new(tos_uri)?;
        let policy_uri = PolicyUri::new(policy_uri)?;
        let auth_method = match auth_method {
            TokenEndPointAuthMethodDto::ClientSecretPost => TokenEndPointAuthMethod::ClientSecretPost,
            TokenEndPointAuthMethodDto::ClientSecretBasic => TokenEndPointAuthMethod::ClientSecretBasic,
            TokenEndPointAuthMethodDto::None => TokenEndPointAuthMethod::None,
            TokenEndPointAuthMethodDto::PrivateKeyJWK => TokenEndPointAuthMethod::PrivateKeyJWK
        };
        let grant_types = grant_types.into_iter()
            .map(|types| match types {
                GrantTypeDto::AuthorizationCode => GrantType::AuthorizationCode,
                GrantTypeDto::Implicit => GrantType::Implicit,
                GrantTypeDto::Password => GrantType::Password,
                GrantTypeDto::ClientCredentials => GrantType::ClientCredentials,
                GrantTypeDto::RefreshToken => GrantType::RefreshToken,
                GrantTypeDto::JWTBearer => GrantType::JWTBearer,
                GrantTypeDto::Saml2Bearer => GrantType::Saml2Bearer
            })
            .collect::<GrantTypes>();
        let response_types = response_types.into_iter()
            .map(|types| match types {
                ResponseTypeDto::Code => ResponseType::Code,
                ResponseTypeDto::Token => ResponseType::Token
            })
            .collect::<ResponseTypes>();
        let scopes = scopes.into_iter()
            .map(|scope| (
                ScopeMethod::new(scope.method),
                ScopeDescription::new(scope.description)
            ))
            .collect::<Scopes>();
        let contacts = contacts.into_iter()
            .map(Address::new)
            .collect::<Contacts>();

        let client = Client::new(
            client_id,
            name,
            client_uri,
            client_desc,
            types,
            logo_uri,
            tos_uri,
            owner.id,
            policy_uri,
            auth_method,
            grant_types,
            response_types,
            scopes,
            contacts,
            jwk
        )?;

        self.registry.register(&client).await?;

        Ok(client.into())
    }
}