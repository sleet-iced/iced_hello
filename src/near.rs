use reqwest;
use serde_json::json;
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
        let args_base64 = base64::encode(json!({}).to_string());

        let query = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "query",
            "params": {
                "request_type": "call_function",
                "finality": "final",
                "account_id": self.config.contract_id,
                "method_name": "get_greeting",
                "args_base64": args_base64
            }
        });

        let response = self.client
            .post(&self.config.get_rpc_url())
            .json(&query)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let result = response["result"]
            .as_array()
            .ok_or("Invalid response format")?;

        let greeting = result
            .iter()
            .map(|n| n.as_u64().unwrap_or(0) as u8 as char)
            .collect::<String>();
            
        Ok(greeting)
    }
}