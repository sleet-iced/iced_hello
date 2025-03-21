use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub network_id: String,
    pub node_url: String,
    pub contract_id: String,
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