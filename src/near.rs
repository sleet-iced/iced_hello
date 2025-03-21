use near_jsonrpc_client::JsonRpcClient;
use near_primitives::types::FunctionArgs;
use near_primitives::views::QueryRequest;
use serde_json::json;
use crate::config::NetworkConfig;

pub struct NearClient {
    config: NetworkConfig,
    rpc_client: JsonRpcClient,
}

impl NearClient {
    pub fn new() -> Self {
        let config = NetworkConfig::default();
        let rpc_client = JsonRpcClient::connect(&config.node_url);
        Self { config, rpc_client }
    }

    pub async fn get_greeting(&self) -> Result<String, Box<dyn std::error::Error>> {
        let args = json!({}).to_string().into_bytes();
        let query = QueryRequest::CallFunction {
            account_id: self.config.contract_id.parse()?,
            method_name: "get_greeting".to_string(),
            args: FunctionArgs::from(args),
        };

        let result = self.rpc_client.call(query).await?;
        let greeting: String = serde_json::from_slice(&result.result)?;
        Ok(greeting)
    }
}