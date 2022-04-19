use ff::PrimeField;
use poseidon_rs::Fr;
use poseidon_rs::Poseidon;

pub fn hash(v1: Fr, v2: Fr) -> Fr {
    let mut big_arr: Vec<Fr> = Vec::new();
    big_arr.push(v1);
    big_arr.push(v2);
    let poseidon = Poseidon::new();
    poseidon.hash(big_arr).unwrap()
}

pub fn generate_merkle_root(leaves: Vec<Fr>) -> Fr {
    let mut leaves = leaves.clone();
    let tree_height = ((leaves.len() as f32).log2().round()) as usize;
    println!("Log2 amount of leaves: {}", tree_height);

    for i in 0..(((leaves.len() as f32).log2().round()) as usize) {
        //For max_options
        for j in 0..leaves.len() {
            let step = 2 << i;
            println!("Step: {}", step);
            // This is the modulus
            let should_skip = j - (j / 2) * 2;
            if (((j + 1) * step) < 8) && (should_skip == 0) {
                leaves[j * step] = hash(leaves[j * step], leaves[(j + 1) * step]);
                println!("Hash of height {} at {}: {}", i, j * step, leaves[j * step]);
            }
        }
    }

    leaves[0]
}
#[cfg(test)]
mod tests {
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
            "Fr(0x0ee79c570dd490a23b9d19037e0b20c5390b1cae9a6fd1c421233ca41408d396)"
        );
    }
}
