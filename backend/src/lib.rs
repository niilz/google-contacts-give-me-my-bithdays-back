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
pub struct Connections {
    connections: Vec<Person>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Person {
    #[serde(rename = "resourceName")]
    resource_name: String,
    etag: String,
    names: Vec<Name>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Name {
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "familyName")]
    family_name: Option<String>,
    #[serde(rename = "unstructuredName")]
    unstructured_name: String,
}
