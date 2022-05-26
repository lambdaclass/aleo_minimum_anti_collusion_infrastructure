use num::{BigUint, Num};
use poseidon_rs::Fr;
use serde::Serialize;

pub fn fr_to_leo_str(fr: Fr) -> String {
    let fr_string = fr.to_string();
    let sliced_string = fr_string[5..(fr_string.len() - 1)].to_string();
    let sliced_str: &str = sliced_string.as_str();
    BigUint::from_str_radix(sliced_str, 16).unwrap().to_string()
}

pub fn fr_vec_to_leo_str(fr_vec: &Vec<Fr>) -> String {
    let string_vec: Vec<String> = fr_vec.iter().map(|x| fr_to_leo_str(*x)).collect();
    format!("[{}]", string_vec.join(","))
}

mod tests {
    use ff::PrimeField;

    use super::*;
    #[test]
    fn test_fr_conversion_leo() {
        let fr: Fr = Fr::from_str(&"321").unwrap();
        let fr_str = super::fr_to_leo_str(fr);
        assert_eq!(fr_str, "321");
    }

    #[test]
    fn test_vec_fr_conversion_leo() {
        let one: Fr = Fr::from_str(&"1").unwrap();
        let two: Fr = Fr::from_str(&"2").unwrap();
        let three: Fr = Fr::from_str(&"3").unwrap();

        let fr_vec = vec![one, three, two];
        let fr_str = super::fr_vec_to_leo_str(&fr_vec);
        assert_eq!(fr_str, "[1,3,2]");
    }
}
