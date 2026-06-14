use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    /*
    Some other properties in token response, not used yet
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub refresh_expires_in: u32,
     */
}
