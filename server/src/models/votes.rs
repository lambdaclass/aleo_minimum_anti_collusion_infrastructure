use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Votes {
    transactions: Vec<String>,
}

impl Votes {
    pub fn new(transactions: Vec<String>) -> Self {
        Self { transactions }
    }
}
