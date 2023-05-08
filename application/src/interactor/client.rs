use kernel::{
    repository::{
        ClientRegistry, 
        AccountRepository
    }, 

};
use kernel::entities::{Address, Client, ClientDescription, ClientId, ClientName, ClientSecret, ClientTypes, ClientUri, Contacts, GrantType, GrantTypes, LogoUri, PolicyUri, RedirectUri, RedirectUris, RegistrationAccessToken, RegistrationEndPoint, ResponseType, ResponseTypes, ScopeDescription, ScopeMethod, Scopes, TermsUri, TokenEndPointAuthMethod, UserId};

use crate::{
    adapter::client::RegisterClientAdapter,
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
impl<T1, T2> RegisterClientAdapter for RegisterClientInteractor<T1, T2>
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
            redirect_uris,
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

        let types = if auth_method != TokenEndPointAuthMethodDto::None {
            ClientTypes::new(ClientSecret::default())
        } else {
            ClientTypes::new(None)
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

        let redirect_uris = redirect_uris.into_iter()
            .map(RedirectUri::new)
            .collect::<RedirectUris>();

        let scopes = scopes.into_iter()
            .map(|scope| (
                ScopeMethod::new(scope.method),
                ScopeDescription::new(scope.description)
            ))
            .collect::<Scopes>();

        let contacts = contacts.into_iter()
            .map(Address::new)
            .collect::<Contacts>();

        let conf_access_token = RegistrationAccessToken::default();
        let conf_endpoint = RegistrationEndPoint::default();

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
            redirect_uris,
            scopes,
            contacts,
            jwk,
            conf_access_token,
            conf_endpoint
        )?;

        self.registry.register(&client).await?;

        Ok(client.into())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use mockall::predicate::always;
    use kernel::entities::Account;
    use kernel::external::{OffsetDateTime, Uuid};
    use kernel::repository::{MockAccountRepository, MockClientRegistry};
    use crate::adapter::client::RegisterClientAdapter;
    use crate::interactor::RegisterClientInteractor;
    use crate::transfer::client::{GrantTypeDto, RegisterClientDto, ResponseTypeDto, ScopeDto, TokenEndPointAuthMethodDto};

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let mut mock_accounts_repository = MockAccountRepository::new();

        let user_id = Uuid::new_v4();
        let address = "test.user@example.com";
        let name = "TEST MAN";
        let pass = "test0000pAssw0rd";
        let created_at = OffsetDateTime::now_utc();
        let updated_at = OffsetDateTime::now_utc();
        let verified_at = OffsetDateTime::now_utc() - Duration::from_secs(80000);

        mock_accounts_repository.expect_find_by_id()
            .with(always())
            .returning(move |_| {
                Ok(Some(Account::new(
                    user_id,
                    address,
                    name,
                    pass,
                    created_at,
                    updated_at,
                    verified_at
                ).unwrap()))
            });

        let mut mock_client_registry = MockClientRegistry::new();

        mock_client_registry.expect_register()
            .with(always())
            .returning(move |v| {
                println!("{:#?}", v);
                Ok(())
            });

        let client_registration = RegisterClientInteractor::new(
            mock_client_registry, mock_accounts_repository
        );

        let client_id = Uuid::new_v4();
        let client_name = "Test Client";
        let client_uri = "https://test.client.example.com/";
        let client_desc = "TEST CLIENT!";
        let logo_uri = "https://test.client.example.com/logo";
        let tos_uri = "https://test.client.example.com/terms";
        let owner_id = user_id;
        let policy_uri = "https://test.client.example.com/policy";
        let auth_method = TokenEndPointAuthMethodDto::ClientSecretPost;
        let grant_types = vec![GrantTypeDto::AuthorizationCode];
        let response_types = vec![ResponseTypeDto::Code];
        let redirect_uris = vec![
            "https://test.client.example.com/callback",
            "https://test.client.example.com/callback2"
        ].into_iter()
         .map(Into::into)
         .collect::<Vec<String>>();
        let scopes = vec![
            ("read", Some("base user data read")),
            ("write", Some("base user data write")),
            ("phantom", None)
        ].into_iter()
         .map(|(method, desc)| (method.to_string(), desc.map(ToOwned::to_owned)))
         .map(|(method, desc)| ScopeDto { method, description: desc })
         .collect::<Vec<ScopeDto>>();
        let contacts = vec!["test.user@client.com"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect::<Vec<String>>();

        let dto = RegisterClientDto {
            id: client_id,
            name: client_name.into(),
            client_uri: client_uri.into(),
            description: client_desc.into(),
            logo_uri: logo_uri.into(),
            tos_uri: tos_uri.into(),
            owner_id,
            policy_uri: policy_uri.into(),
            auth_method,
            grant_types,
            response_types,
            redirect_uris,
            scopes,
            contacts,
            jwk: None,
        };

        let regi = client_registration.register(dto).await?;

        println!("{:#?}", regi);

        Ok(())
    }
}