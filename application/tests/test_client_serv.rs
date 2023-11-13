use application::interactor::{RegisterClientInteractor, UpdateClientInteractor};
use application::services::{RegisterClientService, UpdateClientService};
use application::transfer::client::{
    ClientDto, GrantTypeDto, RegisterClientDto, ResponseTypeDto, ScopeDto,
    TokenEndPointAuthMethodDto, UpdateClientDto,
};
use kernel::external::{OffsetDateTime, Uuid};
use kernel::interfaces::repository::{ClientRegistry, MockAccountRepository, MockClientRegistry};
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

#[tokio::test]
//noinspection DuplicatedCode
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

#[tokio::test]
//noinspection DuplicatedCode
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

    let _before: ClientDto = mock_client_registry
        .find_by_id(&ClientId::new_at_now(Uuid::new_v4()))
        .await?
        .unwrap()
        .into();

    let interactor = UpdateClientInteractor::new(mock_client_registry, mock_accounts_repository);

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

    let _after = interactor
        .update(&Uuid::new_v4(), "none", "test0000pAssw0rd", update)
        .await?;

    assert_ne!(_before, _after);

    Ok(())
}
