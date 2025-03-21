use serde_json::json;
use reqwest;
use base64;
use crate::config::NetworkConfig;

#[derive(Debug, Clone)]
pub struct NearClient {
    config: NetworkConfig,
    client: reqwest::Client,
}

impl NearClient {
    pub fn new() -> Self {
        let config = NetworkConfig::default();
        let client = reqwest::Client::new();
        Self { config, client }
    }

    pub async fn get_greeting(&self) -> Result<String, Box<dyn std::error::Error>> {
        let fastnear_url = format!("{}/account/{}/view/get_greeting", 
            self.config.get_fastnear_url(),
            self.config.contract_id
        );

        let response = self.client
            .get(&fastnear_url)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}