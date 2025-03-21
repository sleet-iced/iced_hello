use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub network_id: String,
    pub contract_id: String,
}

impl NetworkConfig {
    pub fn get_rpc_url(&self) -> String {
        match self.network_id.as_str() {
            "mainnet" => String::from("https://rpc.web4.near.page"),
            _ => String::from("https://rpc.web4.testnet.page")
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            network_id: String::from("testnet"),
            contract_id: String::from("hello.sleet.testnet"),
        }
    }
}