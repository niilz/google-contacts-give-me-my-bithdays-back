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
