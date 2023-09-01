use kernel::prelude::services::JwkSelectionService;
use kernel::{
    external::Uuid,
    interfaces::repository::{
        AccountRepository, ClientRegistry, DependOnAccountRepository, DependOnClientRegistry,
    },
    prelude::entities::{
        Address, Client, ClientDescription, ClientId, ClientName, ClientSecret, ClientTypes,
        ClientUri, Contacts, GrantType, GrantTypes, Jwks, LogoUri, PolicyUri, RedirectUri,
        RedirectUris, RegistrationAccessToken, RegistrationEndPoint, ResponseType, ResponseTypes,
        ScopeDescription, ScopeMethod, Scopes, TermsUri, TokenEndPointAuthMethod, UserId,
    },
};

use crate::services::DeleteClientService;
use crate::{
    services::{RegisterClientService, UpdateClientService},
    transfer::client::{
        ClientDto, GrantTypeDto, RegisterClientDto, ResponseTypeDto, TokenEndPointAuthMethodDto,
        UpdateClientDto,
    },
    ApplicationError,
};

#[derive(Clone)]
pub struct RegisterClientInteractor<C, A> {
    registry: C,
    repository: A,
}

impl<C, A> RegisterClientInteractor<C, A>
where
    C: ClientRegistry,
    A: AccountRepository,
{
    pub fn new(registry: C, repository: A) -> Self {
        Self {
            registry,
            repository,
        }
    }
}

impl<C, A> DependOnClientRegistry for RegisterClientInteractor<C, A>
where
    C: ClientRegistry,
    A: AccountRepository,
{
    type ClientRegistry = C;

    fn client_registry(&self) -> &Self::ClientRegistry {
        &self.registry
    }
}

impl<C, A> DependOnAccountRepository for RegisterClientInteractor<C, A>
where
    C: ClientRegistry,
    A: AccountRepository,
{
    type AccountRepository = A;

    fn account_repository(&self) -> &Self::AccountRepository {
        &self.repository
    }
}

#[async_trait::async_trait]
impl<C, A> RegisterClientService for RegisterClientInteractor<C, A>
where
    C: ClientRegistry,
    A: AccountRepository,
{
    //noinspection DuplicatedCode
    async fn register(&self, register: RegisterClientDto) -> Result<ClientDto, ApplicationError> {
        let RegisterClientDto {
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
            jwks,
            jwks_uri,
        } = register;

        let owner = UserId::new(owner_id);

        let Some(owner) = self.account_repository().find_by_id(&owner).await? else {
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

        let client_id = ClientId::new_at_now(Uuid::new_v4());
        let name = ClientName::new(name);
        let client_uri = ClientUri::new(client_uri)?;
        let client_desc = ClientDescription::new(description);
        let logo_uri = LogoUri::new(logo_uri)?;
        let tos_uri = TermsUri::new(tos_uri)?;
        let policy_uri = PolicyUri::new(policy_uri)?;
        let auth_method = match auth_method {
            TokenEndPointAuthMethodDto::ClientSecretPost => {
                TokenEndPointAuthMethod::ClientSecretPost
            }
            TokenEndPointAuthMethodDto::ClientSecretBasic => {
                TokenEndPointAuthMethod::ClientSecretBasic
            }
            TokenEndPointAuthMethodDto::None => TokenEndPointAuthMethod::None,
            TokenEndPointAuthMethodDto::PrivateKeyJWT => TokenEndPointAuthMethod::PrivateKeyJWT,
        };
        let grant_types = grant_types
            .into_iter()
            .map(|types| match types {
                GrantTypeDto::AuthorizationCode => GrantType::AuthorizationCode,
                GrantTypeDto::Implicit => GrantType::Implicit,
                GrantTypeDto::Password => GrantType::Password,
                GrantTypeDto::ClientCredentials => GrantType::ClientCredentials,
                GrantTypeDto::RefreshToken => GrantType::RefreshToken,
                GrantTypeDto::JWTBearer => GrantType::JWTBearer,
                GrantTypeDto::Saml2Bearer => GrantType::Saml2Bearer,
            })
            .collect::<GrantTypes>();

        let response_types = response_types
            .into_iter()
            .map(|types| match types {
                ResponseTypeDto::Code => ResponseType::Code,
                ResponseTypeDto::Token => ResponseType::Token,
            })
            .collect::<ResponseTypes>();

        let redirect_uris = redirect_uris
            .into_iter()
            .map(RedirectUri::new)
            .collect::<RedirectUris>();

        let scopes = scopes
            .into_iter()
            .map(|scope| {
                (
                    ScopeMethod::new(scope.method),
                    ScopeDescription::new(scope.description),
                )
            })
            .collect::<Scopes>();

        let contacts = contacts.into_iter().map(Address::new).collect::<Contacts>();

        let jwks = JwkSelectionService::check(jwks, jwks_uri)?;

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
            jwks,
            conf_access_token,
            conf_endpoint,
        )?;

        self.client_registry().register(&client).await?;

        Ok(client.into())
    }
}

#[derive(Clone)]
pub struct UpdateClientInteractor<C, A> {
    registry: C,
    accounts: A,
}

impl<C, A> UpdateClientInteractor<C, A> {
    pub fn new(registry: C, accounts: A) -> Self {
        Self { registry, accounts }
    }
}

impl<C, A> DependOnClientRegistry for UpdateClientInteractor<C, A>
where
    A: AccountRepository,
    C: ClientRegistry,
{
    type ClientRegistry = C;

    fn client_registry(&self) -> &Self::ClientRegistry {
        &self.registry
    }
}

impl<C, A> DependOnAccountRepository for UpdateClientInteractor<C, A>
where
    A: AccountRepository,
    C: ClientRegistry,
{
    type AccountRepository = A;

    fn account_repository(&self) -> &Self::AccountRepository {
        &self.accounts
    }
}

#[async_trait::async_trait]
impl<C, A> UpdateClientService for UpdateClientInteractor<C, A>
where
    C: ClientRegistry,
    A: AccountRepository,
{
    //noinspection DuplicatedCode
    async fn update(
        &self,
        id: &Uuid,
        cl_secret: &str,
        pass_phrase: &str,
        update: UpdateClientDto,
    ) -> Result<ClientDto, ApplicationError> {
        let client_id = ClientId::new_at_now(*id);

        let Some(client) = self.client_registry().find_by_id(&client_id).await? else {
            return Err(ApplicationError::NotFound {
                method: "find_by_id",
                entity: "client",
                id: client_id.to_string(),
            })
        };

        if let ClientTypes::Confidential(secret) = client.types() {
            if let Err(e) = secret.verify(cl_secret) {
                return Err(ApplicationError::Verification {
                    method: "client_secret_verify",
                    entity: "client",
                    id: format!("{:?}, `in kernel`: {:?}", client_id, e),
                });
            }
        }

        let Some(owner_ac) = self.account_repository().find_by_id(client.owner()).await? else {
            return Err(ApplicationError::NotFound {
                method: "find_by_id",
                entity: "account",
                id: client.owner().to_string(),
            })
        };

        if let Err(e) = owner_ac.pass().verify(pass_phrase) {
            return Err(ApplicationError::Verification {
                method: "account_password_verify",
                entity: "account",
                id: format!("{:?}, `in kernel`: {:?}", owner_ac.id(), e),
            });
        }

        let mut before = client.into_destruct();

        let UpdateClientDto {
            name,
            client_uri,
            description,
            logo_uri,
            tos_uri,
            owner,
            policy_uri,
            auth_method,
            grant_types,
            response_types,
            redirect_uris,
            scopes,
            contacts,
            jwks,
        } = update;

        before.name = ClientName::new(name);
        before.uri = ClientUri::new(client_uri)?;
        before.desc = ClientDescription::new(description);
        before.logo = LogoUri::new(logo_uri)?;
        before.terms = TermsUri::new(tos_uri)?;
        before.owner = UserId::new(owner);
        before.policy = PolicyUri::new(policy_uri)?;

        before.auth_method = match auth_method {
            TokenEndPointAuthMethodDto::ClientSecretPost => {
                TokenEndPointAuthMethod::ClientSecretPost
            }
            TokenEndPointAuthMethodDto::ClientSecretBasic => {
                TokenEndPointAuthMethod::ClientSecretBasic
            }
            TokenEndPointAuthMethodDto::None => TokenEndPointAuthMethod::None,
            TokenEndPointAuthMethodDto::PrivateKeyJWT => TokenEndPointAuthMethod::PrivateKeyJWT,
        };

        before.grant_types = grant_types
            .into_iter()
            .map(|types| match types {
                GrantTypeDto::AuthorizationCode => GrantType::AuthorizationCode,
                GrantTypeDto::Implicit => GrantType::Implicit,
                GrantTypeDto::Password => GrantType::Password,
                GrantTypeDto::ClientCredentials => GrantType::ClientCredentials,
                GrantTypeDto::RefreshToken => GrantType::RefreshToken,
                GrantTypeDto::JWTBearer => GrantType::JWTBearer,
                GrantTypeDto::Saml2Bearer => GrantType::Saml2Bearer,
            })
            .collect::<GrantTypes>();

        before.response_types = response_types
            .into_iter()
            .map(|types| match types {
                ResponseTypeDto::Code => ResponseType::Code,
                ResponseTypeDto::Token => ResponseType::Token,
            })
            .collect::<ResponseTypes>();

        before.redirect_uris = redirect_uris
            .into_iter()
            .map(RedirectUri::new)
            .collect::<RedirectUris>();

        before.scopes = scopes
            .into_iter()
            .map(|scope| {
                (
                    ScopeMethod::new(scope.method),
                    ScopeDescription::new(scope.description),
                )
            })
            .collect::<Scopes>();

        before.contact = contacts.into_iter().map(Address::new).collect::<Contacts>();

        before.jwks = jwks.map(Jwks::new).transpose()?;

        let after = before.freeze();

        self.client_registry().update(&after).await?;

        Ok(after.into())
    }
}

// Default Impl
impl<T> DeleteClientService for T where T: DependOnClientRegistry + DependOnAccountRepository {}

#[cfg(test)]
mod tests {
    use crate::interactor::{RegisterClientInteractor, UpdateClientInteractor};
    use crate::services::{RegisterClientService, UpdateClientService};
    use crate::transfer::client::{
        ClientDto, GrantTypeDto, RegisterClientDto, ResponseTypeDto, ScopeDto,
        TokenEndPointAuthMethodDto, UpdateClientDto,
    };
    use kernel::external::{OffsetDateTime, Uuid};
    use kernel::interfaces::repository::{
        ClientRegistry, MockAccountRepository, MockClientRegistry,
    };
    use kernel::prelude::entities::{
        Account, Address, Client, ClientId, ClientTypes, GrantType, RedirectUri,
        RegistrationAccessToken, RegistrationEndPoint, ResponseType, ScopeDescription, ScopeMethod,
        TokenEndPointAuthMethod,
    };
    use mockall::predicate::always;
    use std::time::Duration;

    fn new_mock_accounts_repo() -> MockAccountRepository {
        let mut mock_accounts_repository = MockAccountRepository::new();

        let user_id = Uuid::new_v4();
        let address = "test.user@example.com";
        let name = "TEST MAN";
        let pass = "test0000pAssw0rd";
        let created_at = OffsetDateTime::now_utc();
        let updated_at = OffsetDateTime::now_utc();
        let verified_at = OffsetDateTime::now_utc() - Duration::from_secs(80000);

        mock_accounts_repository
            .expect_find_by_id()
            .with(always())
            .returning(move |_| {
                Ok(Some(
                    Account::new(
                        user_id,
                        address,
                        name,
                        pass,
                        created_at,
                        updated_at,
                        verified_at,
                    )
                    .unwrap(),
                ))
            });

        mock_accounts_repository
    }

    //noinspection DuplicatedCode
    #[tokio::test]
    async fn test_register() -> anyhow::Result<()> {
        let mock_accounts_repository = new_mock_accounts_repo();

        let mut mock_client_registry = MockClientRegistry::new();

        mock_client_registry
            .expect_register()
            .with(always())
            .returning(move |v| {
                println!("{:#?}", v);
                Ok(())
            });

        let client_registration =
            RegisterClientInteractor::new(mock_client_registry, mock_accounts_repository);

        let client_name = "Test Client";
        let client_uri = "https://test.client.example.com/";
        let client_desc = "TEST CLIENT!";
        let logo_uri = "https://test.client.example.com/logo";
        let tos_uri = "https://test.client.example.com/terms";
        let owner_id = Uuid::new_v4();
        let policy_uri = "https://test.client.example.com/policy";
        let auth_method = TokenEndPointAuthMethodDto::ClientSecretPost;
        let grant_types = vec![GrantTypeDto::AuthorizationCode];
        let response_types = vec![ResponseTypeDto::Code];
        let redirect_uris = vec![
            "https://test.client.example.com/callback",
            "https://test.client.example.com/callback2",
        ]
        .into_iter()
        .map(Into::into)
        .collect::<Vec<String>>();
        let scopes = vec![
            ("read", Some("base user data read")),
            ("write", Some("base user data write")),
            ("phantom", None),
        ]
        .into_iter()
        .map(|(method, desc)| (method.to_string(), desc.map(ToOwned::to_owned)))
        .map(|(method, desc)| ScopeDto {
            method,
            description: desc,
        })
        .collect::<Vec<ScopeDto>>();
        let contacts = vec!["test.user@client.com"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect::<Vec<String>>();

        let jwks_uri = Some("https://stellar.example.com/.well-known".to_string());

        let dto = RegisterClientDto {
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
            jwks: None,
            jwks_uri,
        };

        let regi = client_registration.register(dto).await?;

        println!("{:#?}", regi);

        Ok(())
    }

    //noinspection DuplicatedCode
    #[tokio::test]
    async fn test_update() -> anyhow::Result<()> {
        let mock_accounts_repository = new_mock_accounts_repo();

        let mut mock_client_registry = MockClientRegistry::new();

        mock_client_registry
            .expect_find_by_id()
            .with(always())
            .returning(move |_| {
                let client_id = ClientId::new_at_now(Uuid::new_v4());
                let client_name = "Test Client";
                let client_uri = "https://test.client.example.com/";
                let client_desc = "TEST CLIENT!";
                let client_type = ClientTypes::Public;
                let logo_uri = "https://test.client.example.com/logo";
                let tos_uri = "https://test.client.example.com/terms";
                let owner_id = Uuid::new_v4();
                let policy_uri = "https://test.client.example.com/policy";
                let auth_method = TokenEndPointAuthMethod::ClientSecretPost;
                let grant_types = vec![GrantType::AuthorizationCode];
                let response_types = vec![ResponseType::Code];
                let redirect_uris = vec![
                    "https://test.client.example.com/callback",
                    "https://test.client.example.com/callback2",
                ]
                .into_iter()
                .map(RedirectUri::new)
                .collect::<Vec<RedirectUri>>();
                let scopes = vec![
                    ("read", Some("base user data read")),
                    ("write", Some("base user data write")),
                    ("phantom", None),
                ]
                .into_iter()
                .map(|(method, desc)| (method.to_string(), desc.map(ToOwned::to_owned)))
                .map(|(method, desc)| (ScopeMethod::new(method), ScopeDescription::new(desc)))
                .collect::<Vec<(ScopeMethod, ScopeDescription)>>();
                let contacts = vec!["test.user@client.com"]
                    .into_iter()
                    .map(Address::new)
                    .collect::<Vec<Address>>();
                let reg_token = RegistrationAccessToken::default();
                let reg_endpoint = RegistrationEndPoint::default();
                let client = Client::new(
                    client_id,
                    client_name,
                    client_uri,
                    client_desc,
                    client_type,
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
                    None,
                    reg_token,
                    reg_endpoint,
                )
                .unwrap();

                println!("{:#?}", client);
                Ok(Some(client))
            });

        mock_client_registry
            .expect_update()
            .with(always())
            .returning(move |v| {
                println!("{:#?}", v);
                Ok(())
            });

        let before: ClientDto = mock_client_registry
            .find_by_id(&ClientId::new_at_now(Uuid::new_v4()))
            .await?
            .unwrap()
            .into();

        let interactor =
            UpdateClientInteractor::new(mock_client_registry, mock_accounts_repository);

        let update = UpdateClientDto {
            name: "TEST CLIENT MK2".to_string(),
            client_uri: "https://client.test.com/".to_string(),
            description: "TEST 2".to_string(),
            logo_uri: "https://logo.example.com".to_string(),
            tos_uri: "https://client.test.com/terms".to_string(),
            owner: Default::default(),
            policy_uri: "https://policy.example.com/".to_string(),
            auth_method: TokenEndPointAuthMethodDto::None,
            grant_types: vec![GrantTypeDto::AuthorizationCode, GrantTypeDto::Implicit],
            response_types: vec![ResponseTypeDto::Token],
            redirect_uris: vec!["https://client.test.com/callback"]
                .into_iter()
                .map(Into::into)
                .collect(),
            scopes: vec![
                ("read", Some("base user data read")),
                ("write", Some("base user data write")),
                ("phantom", None),
            ]
            .into_iter()
            .map(|(method, desc)| (method.to_string(), desc.map(ToOwned::to_owned)))
            .map(|(method, desc)| ScopeDto {
                method,
                description: desc,
            })
            .collect::<Vec<ScopeDto>>(),
            contacts: vec!["test.user@client.com"]
                .into_iter()
                .map(ToOwned::to_owned)
                .collect::<Vec<String>>(),
            jwks: None,
        };

        let after = interactor
            .update(&Uuid::new_v4(), "none", "test0000pAssw0rd", update)
            .await?;

        assert_ne!(before, after);

        Ok(())
    }
}
