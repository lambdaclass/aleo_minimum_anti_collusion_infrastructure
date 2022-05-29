use poseidon_rs::Fr;
use poseidon_rs::Poseidon;
use std::fmt;

use crate::fr_helpers::fr_to_leo_str;
use crate::fr_helpers::fr_vec_to_leo_str;

pub fn hash(v1: Fr, v2: Fr) -> Fr {
    let big_arr: Vec<Fr> = vec![v1, v2];
    let poseidon = Poseidon::new();
    poseidon.hash(big_arr).unwrap()
}

/// Generates the merkle root of the givven leaves using Poseidon algorithm
/// Compatible with Leo MACI circuits
pub fn generate_merkle_root(mut leaves: Vec<Fr>) -> Fr {
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
#[derive(Debug, Clone)]
pub struct SizeNotPowerOfTwoError;

impl fmt::Display for SizeNotPowerOfTwoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MerkleTree only supports power of two leaves")
    }
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub nodes: Vec<Fr>,
}

/*leaf: &str, proof: &str, path_index: &str, root: &str */
#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub leaf: Fr,
    pub proof_elements: Vec<Fr>,
    pub path_index: Vec<bool>,
}

impl MerkleProof {
    pub fn new(leaf: Fr, proof_elements: Vec<Fr>, path_index: Vec<bool>) -> Self {
        Self {
            leaf,
            proof_elements,
            path_index,
        }
    }

    pub fn to_proof_strings(&self) -> MerkleProofStrings {
        let leaf_string = fr_to_leo_str(self.leaf);
        let proof_string = fr_vec_to_leo_str(&self.proof_elements);

        let path_index_string_vec: Vec<String> =
            self.path_index.iter().map(|x| x.to_string()).collect();
        let path_index_string = format!("[{}]", path_index_string_vec.join(","));

        MerkleProofStrings::new(leaf_string, proof_string, path_index_string)
    }
}
#[derive(Debug, Clone)]
pub struct MerkleProofStrings {
    leaf: String,
    proof_elements: String,
    path_index: String,
}

impl MerkleProofStrings {
    pub fn new(leaf: String, proof_elements: String, path_index: String) -> Self {
        Self {
            leaf,
            proof_elements,
            path_index,
        }
    }

    pub fn leaf(&self) -> String {
        self.leaf.clone()
    }

    pub fn proof_elements(&self) -> String {
        self.proof_elements.clone()
    }

    pub fn path_index(&self) -> String {
        self.path_index.clone()
    }
}

impl MerkleTree {
    fn is_power_of_two(x: usize) -> bool {
        (x & (x - 1)) == 0
    }

    pub fn new(leaves: Vec<Fr>) -> Result<Self, SizeNotPowerOfTwoError> {
        if !Self::is_power_of_two(leaves.len()) {
            return Err(SizeNotPowerOfTwoError);
        }
        let mut nodes = leaves;
        let amount_of_leaves = nodes.len();
        let tree_height = (nodes.len() as f32).log2().round() as usize;
        let mut offset = 0;
        //For each level
        for level in 1..tree_height + 1 {
            if level != 1 {
                offset += amount_of_leaves / (2usize.pow(level as u32 - 2));
            }
            //For each node in the level
            for j in 0..(amount_of_leaves / (2usize.pow(level as u32))) {
                let new_node = hash(nodes[j * 2 + offset], nodes[j * 2 + 1 + offset]);
                nodes.push(new_node);
            }
        }

        Ok(Self { nodes })
    }

    pub fn merkle_proof_for(&self, element_index: usize) -> MerkleProof {
        let amount_leaves = (self.nodes.len() / 2) + 1;
        let mut vec_proof: Vec<Fr> = Vec::new();
        let mut path: Vec<bool> = Vec::new();
        let mut width = amount_leaves;
        let mut j = element_index;
        let mut base = 0;

        while self.nodes.len() > base + 1 {
            let hash_index = (j / 2) * 2;
            for k in hash_index..hash_index + 2 {
                if k != j {
                    vec_proof.push(self.nodes[base + k]);
                }
            }
            path.push(j % 2 != 0); // path_index

            base += width;
            width >>= 1; // width /= 2;
            j >>= 1; // j /= 2;
        }

        // vec_proof.push(*self.nodes.last().unwrap());

        //TO DO: Update merkle path index
        MerkleProof::new(self.nodes[element_index], vec_proof, path)
    }

    pub fn root(&self) -> Fr {
        *self.nodes.last().unwrap()
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
            "Fr(0x0ee79c570dd490a23b9d19037e0b20c5390b1cae9a6fd1c421233ca41408d396)"
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

    #[test]
    fn merkle_proof_test() {
        let one: Fr = Fr::from_str("1").unwrap();
        let two: Fr = Fr::from_str("2").unwrap();
        let three: Fr = Fr::from_str("3").unwrap();
        let four: Fr = Fr::from_str("4").unwrap();
        let five: Fr = Fr::from_str("5").unwrap();

        let votes: [Fr; 32] = [
            one, two, four, five, two, three, one, two, one, two, three, two, two, three, one, two,
            one, two, three, two, two, three, one, two, one, two, three, two, two, three, one, two,
        ];

        let tree = MerkleTree::new(votes.to_vec()).unwrap();

        let proof0 = tree.merkle_proof_for(0);

        assert_eq!(proof0.proof_elements[0], two);

        assert_eq!(&proof0.proof_elements[1], &hash(four, five));
    }

    #[test]
    fn leo_representation_test() {
        let one: Fr = Fr::from_str("1").unwrap();
        let two: Fr = Fr::from_str("2").unwrap();
        let three: Fr = Fr::from_str("3").unwrap();
        let four: Fr = Fr::from_str("4").unwrap();
        let five: Fr = Fr::from_str("5").unwrap();

        let mock_proof_elements = vec![one, two, four, three];
        let mock_leaf = five;
        let mock_index_path = vec![true, false, false, true, true];
        let proof_strings =
            MerkleProof::new(mock_leaf, mock_proof_elements, mock_index_path).to_proof_strings();

        assert_eq!(proof_strings.leaf, "5");
        assert_eq!(proof_strings.proof_elements, "[1,2,4,3]");
        assert_eq!(proof_strings.path_index, "[true,false,false,true,true]");
    }
}
