mod auth;
mod config;
mod models;

use crate::{auth::token::get_token, config::Config, models::ai_model};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = Config::from_env()?;

    let kc_token = get_token(&config)?;

    let bearer = format!("Bearer {}", kc_token.access_token);

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
