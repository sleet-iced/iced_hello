use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Config {
    contract: String,
    rpc_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("config/config.json");

    let config: Config = serde_json::from_reader(
        std::fs::File::open(&config_path)
            .with_context(|| format!("Failed to open config at {:?}", config_path))?
    ).context("Failed to parse config")?;

    let rpc_url = &config.rpc_url;

    let response = reqwest::Client::new()
        .post(rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "query",
            "params": {
                "request_type": "call_function",
                "account_id": config.contract,
                "method_name": "get_greeting",
                "args_base64": "",
                "finality": "final"
            }
        }))
        .send()
        .await
        .with_context(|| "RPC call failed")?;

    let response_text = response.text().await.with_context(|| "Failed to get response text")?;
    #[derive(Debug, serde::Deserialize)]
    struct RpcResponse {
        result: RpcResult,
    }

    #[derive(Debug, serde::Deserialize)]
    struct RpcResult {
        result: Vec<u8>,
    }

    println!("Raw response: {}", response_text);

    let response_data: RpcResponse = serde_json::from_str(&response_text).with_context(|| "Failed to parse JSON response")?;
    let greeting_bytes = response_data.result.result;
    let greeting = String::from_utf8_lossy(&greeting_bytes).into_owned();
    println!("Contract greeting: {}", greeting);
    Ok(())
}