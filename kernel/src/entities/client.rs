use destructure::Destructure;
use serde::{Serialize, Deserialize};

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
    jwks: Jwks
}