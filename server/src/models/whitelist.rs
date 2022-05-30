use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Whitelist {
    // In the future this should be a pubkey
    pub accounts: Vec<String>,
}

impl Whitelist {
    pub fn new(accounts: Vec<String>) -> Self {
        Self { accounts }
    }
}
