use near_api::NearApi;
use serde_json::json;
use crate::config::NetworkConfig;

pub struct NearClient {
    config: NetworkConfig,
    api: NearApi,
}

impl NearClient {
    pub fn new() -> Self {
        let config = NetworkConfig::default();
        let api = NearApi::new(&config.node_url);
        Self { config, api }
    }

    pub async fn get_greeting(&self) -> Result<String, Box<dyn std::error::Error>> {
        let args = json!({});
        let result = self.api.view_function(
            &self.config.contract_id,
            "get_greeting",
            args
        ).await?;
        
        let greeting: String = serde_json::from_value(result)?;
        Ok(greeting)
    }
}