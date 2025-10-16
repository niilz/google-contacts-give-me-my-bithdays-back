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
    pub connections: Vec<Person>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Person {
    #[serde(rename = "resourceName")]
    resource_name: String,
    etag: String,
    pub names: Vec<Name>,
    pub birthdays: Option<Vec<Birthday>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Name {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "familyName")]
    pub family_name: Option<String>,
    #[serde(rename = "unstructuredName")]
    pub unstructured_name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Birthday {
    pub date: Date,
    pub text: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Date {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
}
