use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub network_id: String,
    pub node_url: String,
    pub contract_id: String,
}

impl NetworkConfig {
    pub fn get_fastnear_url(&self) -> String {
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
            node_url: String::from("https://rpc.testnet.near.org"),
            contract_id: String::from("hello.sleet.testnet"),
        }
    }
}