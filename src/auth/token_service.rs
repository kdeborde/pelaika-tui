use crate::models::token::TokenResponse;

pub fn get_token(
    kc_token_endpoint: &str,
    kc_client_id: &str,
    username: &str,
    password: &str,
) -> Result<TokenResponse, Box<dyn std::error::Error>> {
    let req_body = [
        ("grant_type", "password"),
        ("client_id", kc_client_id),
        ("username", username),
        ("password", password),
        ("scope", "openid"),
    ];

    let body = ureq::post(kc_token_endpoint)
        .send_form(req_body)?
        .body_mut()
        .read_to_string()?;

    let parsed: TokenResponse = serde_json::from_str(&body)?;
    Ok(parsed)
}
