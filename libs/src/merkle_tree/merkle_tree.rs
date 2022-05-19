use poseidon_rs::Fr;
use poseidon_rs::Poseidon;
use std::fmt;

pub fn hash(v1: Fr, v2: Fr) -> Fr {
    let mut big_arr: Vec<Fr> = Vec::new();
    big_arr.push(v1);
    big_arr.push(v2);
    let poseidon = Poseidon::new();
    poseidon.hash(big_arr).unwrap()
}

/// Generates the merkle root of the givven leaves using Poseidon algorithm
/// Compatible with Leo MACI circuits
pub fn generate_merkle_root(leaves: Vec<Fr>) -> Fr {
    let mut leaves = leaves.clone();

    for i in 0..(((leaves.len() as f32).log2().round()) as usize) {
        //For max_options
        for j in 0..leaves.len() {
            let step = if i == 0 { 1 } else { 2 << (i - 1) };
            // This is the modulus
            let should_skip = j - (j / 2) * 2;

            if (((j + 1) * step) < leaves.len()) && (should_skip == 0) {
                leaves[j * step] = hash(leaves[j * step], leaves[(j + 1) * step]);
            }
        }
    }
    leaves[0]
}

fn is_power_of_two(x: usize) -> bool {
    return (x & (x - 1)) == 0;
}

#[derive(Debug, Clone)]
struct SizeNotPowerOfTwoError;

impl fmt::Display for SizeNotPowerOfTwoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MerkleTree only supports power of two leaves")
    }
}

struct MerkleTree {
    pub nodes: Vec<Fr>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<Fr>) -> Result<Self, SizeNotPowerOfTwoError> {
        if !is_power_of_two(leaves.len()) {
            return Err(SizeNotPowerOfTwoError);
        }
        let mut nodes = leaves;
        let amount_of_leaves = nodes.len();
        let tree_height = (nodes.len() as f32).log2().round() as usize;
        let mut offset = 0;
        //For each level
        for level in 1..tree_height + 1 {
            if (level != 1) {
                offset = offset + (amount_of_leaves / (2usize.pow(level as u32 - 2)));
            }
            //For each node in the level
            for j in 0..(amount_of_leaves / (2usize.pow(level as u32))) {
                let new_node = hash(nodes[j * 2 + offset], nodes[j * 2 + 1 + offset]);
                nodes.push(new_node);
            }
        }
        Ok(Self { nodes })
    }
}

#[cfg(test)]
mod tests {
    use ff::PrimeField;

    use super::*;
    #[test]
    fn hash_1_2() {
        assert_eq!(
            hash(Fr::from_str("1").unwrap(), Fr::from_str("2").unwrap()).to_string(),
            "Fr(0x9811D68B946C0FC88B0B7FECCC1C35B792A732B3E072CA864DF3AEE94826684)"
        );
    }

    #[test]
    fn merkle_root_test() {
        let one: Fr = Fr::from_str("1").unwrap();
        let two: Fr = Fr::from_str("2").unwrap();
        let three: Fr = Fr::from_str("3").unwrap();

        let votes: [Fr; 32] = [
            one, two, three, two, two, three, one, two, one, two, three, two, two, three, one, two,
            one, two, three, two, two, three, one, two, one, two, three, two, two, three, one, two,
        ];

        assert_eq!(
            generate_merkle_root(votes.to_vec()).to_string(),
            "Fr(0x0d71cbc322578e133085b861a656d34b3abc2cc65ac11d24618aa53d49e5d443)"
        );
    }

    #[test]
    fn merkle_tree_root_test() {
        let one: Fr = Fr::from_str("1").unwrap();
        let two: Fr = Fr::from_str("2").unwrap();
        let three: Fr = Fr::from_str("3").unwrap();

        let votes: [Fr; 32] = [
            one, two, three, two, two, three, one, two, one, two, three, two, two, three, one, two,
            one, two, three, two, two, three, one, two, one, two, three, two, two, three, one, two,
        ];

        let tree = MerkleTree::new(votes.to_vec()).unwrap();
        let root = tree.nodes.last().unwrap();
        assert_eq!(
            root.to_string(),
            "Fr(0x0d71cbc322578e133085b861a656d34b3abc2cc65ac11d24618aa53d49e5d443)"
        );
    }
}
