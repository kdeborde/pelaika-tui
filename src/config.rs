pub struct Config {
    pub api_base_url: String,
    pub kc_token_endpoint: String,
    pub kc_client_id: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let api_base_url = std::env::var("PELAIKA_API_BASE_URL")?;
        let kc_token_endpoint = std::env::var("PELAIKA_KC_TOKEN_ENDPOINT")?;
        let kc_client_id = std::env::var("PELAIKA_KC_CLIENT_ID")?;
        let username = std::env::var("PELAIKA_USERNAME")?;
        let password = std::env::var("PELAIKA_PASSWORD")?;
        Ok(Self {
            api_base_url,
            kc_token_endpoint,
            kc_client_id,
            username,
            password,
        })
    }
}
