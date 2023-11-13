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
            });
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
            });
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
            });
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
