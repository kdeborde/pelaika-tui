use serde::Deserialize;

#[derive(Deserialize)]
pub struct AiModel {
    pub name: String,
    pub size: u64,
    pub capabilities: Vec<String>,
}
