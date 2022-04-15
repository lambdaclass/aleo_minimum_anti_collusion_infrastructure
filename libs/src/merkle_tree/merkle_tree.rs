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
}
