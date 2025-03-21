use reqwest;
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
            self.config.get_rpc_url(),
            self.config.contract_id
        );

        let response = self.client
            .get(&fastnear_url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        if let Some(result) = response.get("result") {
            if let Some(greeting) = result.as_str() {
                Ok(greeting.to_string())
            } else {
                Err("Invalid greeting format".into())
            }
        } else {
            Err("No result field in response".into())
        }
    }
}