use near_sdk::serde_json::json;
use near_sdk::AccountId;
use reqwest;
use crate::config::NetworkConfig;

#[derive(Debug)]
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
        let contract_id: AccountId = self.config.contract_id.parse()?;
        let args_base64 = base64::encode(json!({}).to_string());

        let query = json!({
            "request_type": "call_function",
            "finality": "final",
            "account_id": contract_id.to_string(),
            "method_name": "get_greeting",
            "args_base64": args_base64
        });

        let response = self.client
            .post(&self.config.node_url)
            .json(&query)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let result = response["result"]["result"]
            .as_array()
            .ok_or("Invalid response format")?;
        
        let greeting = String::from_utf8(base64::decode(result)?)?;
        Ok(greeting)
}