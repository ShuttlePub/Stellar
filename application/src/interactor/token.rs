use std::time::Duration;

use kernel::{
    repository::{
        AuthorizeTokenRepository, 
        ClientRegistry, 
        AccountRepository
    }, 
    entities::{
        ClientId, 
        AuthorizeToken, 
        AuthorizeTokenId, 
        UserId, 
        Scopes,
        ScopeMethod,
        RedirectUri, 
        DestructAccount, 
        ClientTypes,
    }
};
use kernel::external::Uuid;

use crate::{
    ApplicationError,
    transfer::token::{
        CreateAuthorizeTokenDto, 
        AuthorizeTokenDto
    },
    adaptor::token::CreateAuthorizeTokenAdaptor
};

#[derive(Clone)]
pub struct CreateAuthorizeTokenInteractor<T1, T2, T3> {
    kvs: T1,
    registry: T2,
    accounts: T3
}

impl<T1, T2, T3> CreateAuthorizeTokenInteractor<T1, T2, T3> {
    pub fn new(kvs: T1, registry: T2, accounts: T3) -> Self {
        Self { kvs, registry, accounts }
    }
}

#[async_trait::async_trait]
impl<T1, T2, T3> CreateAuthorizeTokenAdaptor for CreateAuthorizeTokenInteractor<T1, T2, T3>
  where T1: AuthorizeTokenRepository,
        T2: ClientRegistry,
        T3: AccountRepository
{
    async fn create(&self, user: &Uuid, create: CreateAuthorizeTokenDto) -> Result<AuthorizeTokenDto, ApplicationError> {
        let CreateAuthorizeTokenDto { 
            response_type, 
            client_id, 
            client_secret, 
            redirect_uri, 
            scope 
        } = create;

        if response_type.as_str() != "code" {
            return Err(ApplicationError::InvalidValue { 
                method: "response_type validation", 
                value: response_type
            })
        }

        let user_id = UserId::new(*user);

        let Some(account) = self.accounts.find_by_id(&user_id).await? else {
            return Err(ApplicationError::NotFound { 
                method: "find",
                entity: "account", 
                id: user_id.to_string()
            })
        };

        let client_id = ClientId::new_at_now(client_id);

        if scope.iter().any(|scoped| scoped == "refresh_token") {
            if let Some(request) = client_secret {
                let Some(client) = self.registry.find_by_id(&client_id).await? else {
                    return Err(ApplicationError::NotFound { 
                        method: "find_by_id", 
                        entity: "client", 
                        id: client_id.to_string() 
                    });
                };
    
                let ClientTypes::Confidential(secret) = client.types() else {
                    return Err(ApplicationError::InvalidValue { 
                        method: "Check if the request is Confidential because it has a `client_secret`", 
                        value: client_id.to_string()
                    });
                };
    
                secret.verify(request)?;
            };
        }

        let DestructAccount { id, .. } = account.into_destruct();

        let account_id = id;
        let id = AuthorizeTokenId::default();
        let created_at = OffsetDateTime::now_utc();
        let updated_at = created_at;
        let client_id = ClientId::new(Uuid::new_v4());
        let mut scopes = Scopes::default();
        for object in scope.into_iter() {
            scopes.add(Method::try_from(object)?)
        }
        let redirect_uri = RedirectUri::new(redirect_uri);
        let expired_in = Duration::from_secs(600);

        let token = AuthorizeToken::new(
            id, 
            created_at, 
            updated_at, 
            account_id, 
            client_id, 
            scopes, 
            redirect_uri, 
            expired_in
        );

        self.kvs.create(&token).await?;

        Ok(token.into())
    }
}