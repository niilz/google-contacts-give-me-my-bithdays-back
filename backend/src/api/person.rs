use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct AuthTokens {
    pub(crate) access_token: String,
    pub(crate) expires_in: u32,
    pub(crate) refresh_token: Option<String>,
    pub(crate) scope: String,
    pub(crate) token_type: String,
    pub(crate) refresh_token_expires_in: Option<u32>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Connections {
    pub(crate) connections: Vec<Person>,
    #[serde(rename = "nextPageToken")]
    pub(crate) next_page_token: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Person {
    #[serde(rename = "resourceName")]
    resource_name: String,
    etag: String,
    pub(crate) names: Vec<Name>,
    pub(crate) birthdays: Option<Vec<Birthday>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Name {
    #[serde(rename = "displayName")]
    pub(crate) display_name: String,
    #[serde(rename = "familyName")]
    pub(crate) family_name: Option<String>,
    #[serde(rename = "unstructuredName")]
    pub(crate) unstructured_name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Birthday {
    pub(crate) date: Date,
    pub(crate) text: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Date {
    pub(crate) day: u32,
    pub(crate) month: u32,
    pub(crate) year: Option<u32>,
}
