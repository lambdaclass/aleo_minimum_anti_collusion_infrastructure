use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    results: Vec<u32>,
}

impl Results {
    pub fn new(results: Vec<u32>) -> Self {
        Self { results }
    }

    pub fn new_from_string(results: Vec<String>) -> Self {
        Self {
            results: results
                .into_iter()
                .map(|r| r.parse::<u32>().unwrap())
                .collect(),
        }
    }
}
