use reqwest;
use serde_json::json;
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
        let url = format!("{}/account/{}/view/get_greeting", 
            self.config.get_rpc_url(),
            self.config.contract_id
        );

        let response = self.client
            .get(&url)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}