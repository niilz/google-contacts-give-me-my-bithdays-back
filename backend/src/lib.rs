use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AuthTokens {
    access_token: String,
    expires_in: u32,
    refresh_token: Option<String>,
    scope: String,
    token_type: String,
    refresh_token_expires_in: Option<u32>,
}

