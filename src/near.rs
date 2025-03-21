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

        let result = response["result"]["result"]
            .as_array()
            .ok_or("Invalid response format")?;
        
        let result_bytes = base64::decode(result
            .iter()
            .map(|v| v.as_u64().unwrap_or(0) as u8)
            .collect::<Vec<u8>>())?;
            
        let greeting = String::from_utf8(result_bytes)?;
        Ok(greeting)
    }
}