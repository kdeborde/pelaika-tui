use crate::config::Config;
use serde::Deserialize;

pub fn get_token(config: &Config) -> Result<TokenResponse, Box<dyn std::error::Error>> {
    let req_body = [
        ("grant_type", "password"),
        ("client_id", &config.kc_client_id),
        ("username", &config.username),
        ("password", &config.password),
        ("scope", "openid"),
    ];

    let body = ureq::post(&config.kc_token_endpoint)
        .send_form(req_body)?
        .body_mut()
        .read_to_string()?;

    let parsed: TokenResponse = serde_json::from_str(&body)?;
    Ok(parsed)
}

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
