use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AuthTokens {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
    pub refresh_token_expires_in: Option<u32>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Birthdays {
    connections: Vec<Connection>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Connection {
    #[serde(rename = "resourceName")]
    resource_name: String,
    etag: String,
    names: Vec<ConnectionData>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct ConnectionData {
    metadata: MetaData,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "familyName")]
    family_name: String,
    #[serde(rename = "givenName")]
    given_name: String,
    #[serde(rename = "displayNameLastFirst")]
    display_name_last_first: String,
    #[serde(rename = "unstructuredName")]
    unstructured_name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct MetaData {
    primary: bool,
    source: Source,
    #[serde(rename = "sourcePrimary")]
    source_primary: bool,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Source {
    #[serde(rename = "sourcePrimary")]
    typ: String,
    id: String,
}
