mod config;
mod models;

use crate::config::Config;
use crate::models::ai_model;
use crate::models::token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = Config::from_env()?;

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

    println!("{body}");
    let parsed: token::TokenResponse = serde_json::from_str(&body)?;
    println!("Parsed successfully");
    let bearer = format!("Bearer {}", parsed.access_token);
    let ai_models = get_available_models(&bearer, &config.api_base_url)?;

    for model in ai_models {
        println!("Name: {}", model.name);
        println!("Size: {}", model.size);
        println!("Capabilities: {:?}", model.capabilities);
    }

    Ok(())
}

fn get_available_models(
    bearer: &str,
    api_url: &str,
) -> Result<Vec<ai_model::AiModel>, Box<dyn std::error::Error>> {
    let models_url = format!("{api_url}/models");
    
    let body = ureq::get(&models_url)
        .header("Authorization", bearer)
        .call()?
        .body_mut()
        .read_to_string()?;

    let available_models: Vec<ai_model::AiModel> = serde_json::from_str(&body)?;

    Ok(available_models)
}
