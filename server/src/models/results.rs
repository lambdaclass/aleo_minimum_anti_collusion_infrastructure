use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    results: Vec<u32>,
}

impl Results {
    pub fn new(results: Vec<u32>) -> Self {
        Self { results }
    }
}
