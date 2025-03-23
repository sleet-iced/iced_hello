use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;
use tokio::runtime::Runtime;

#[derive(Debug, Deserialize)]
struct Config {
    contract: String,
    rpc_url: String,
}

pub fn fetch_and_save_contract_greeting() -> Result<()> {
    let rt = Runtime::new().context("Failed to create Tokio runtime")?;
    rt.block_on(async {
        fetch_and_save_contract_greeting_inner().await
    })
}

async fn fetch_and_save_contract_greeting_inner() -> Result<()> {
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

    let response_data: RpcResponse = serde_json::from_str(&response_text).with_context(|| "Failed to parse JSON response")?;
    let greeting_bytes = response_data.result.result;
    let greeting = String::from_utf8_lossy(&greeting_bytes).into_owned();
    
    let greeting_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/greeting.json");
    let greeting_json = serde_json::json!({"greeting": greeting});
    std::fs::write(&greeting_path, serde_json::to_string_pretty(&greeting_json)?)
        .with_context(|| format!("Failed to write greeting to {:?}", greeting_path))?;

    Ok(())
}