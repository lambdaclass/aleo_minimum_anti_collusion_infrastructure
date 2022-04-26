use aleo_maci_libs::merkle_tree::generate_merkle_root;
use ff::PrimeField;
use poseidon_rs::Fr;
use serde::{Deserialize, Serialize};

use crate::{services::fr_string_to_leo_str, utils::votes_to_fix_array};

pub const NUM_OPTIONS: usize = 8;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tally {
    pub votes: Vec<u32>,
    pub results: Vec<u32>,
    pub votes_markle_root: String,
    pub results_markle_root: String,
}

impl Tally {
    pub fn new(votes: Vec<u32>) -> Self {
        let mut tally = Self {
            votes,
            results: Vec::from([0; NUM_OPTIONS]),
            votes_markle_root: String::from(""),
            results_markle_root: String::from(""),
        };
        tally.generate_votes_markle_root();
        tally.generate_results();
        tally.generate_results_markle_root();
        tally
    }

    pub fn _votes_to_string(&self) -> Vec<String> {
        self.votes
            .clone()
            .into_iter()
            .map(|v| v.to_string())
            .collect()
    }

    fn generate_results(&mut self) {
        self.votes.iter().for_each(|vote| {
            self.results[(vote - 1) as usize] += 1;
        });
    }

    fn generate_votes_markle_root(&mut self) {
        let votes_fixed = votes_to_fix_array(&self.votes);

        let root = generate_merkle_root(
            votes_fixed
                .iter()
                .map(|vote| Fr::from_str(&vote.to_string()).unwrap())
                .collect(),
        )
        .to_string();

        self.votes_markle_root = fr_string_to_leo_str(root);
    }

    fn generate_results_markle_root(&mut self) {
        let root = generate_merkle_root(
            self.results
                .iter()
                .map(|result| Fr::from_str(&result.to_string()).unwrap())
                .collect(),
        )
        .to_string();

        self.results_markle_root = fr_string_to_leo_str(root);
    }
}
