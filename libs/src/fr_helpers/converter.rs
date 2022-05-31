use bech32::FromBase32;
use ff::PrimeField;
use num::{BigUint, Num};
use poseidon_rs::Fr;
use std::error;

pub fn fr_to_leo_str(fr: Fr) -> String {
    let fr_string = fr.to_string();
    let sliced_string = fr_string[5..(fr_string.len() - 1)].to_string();
    let sliced_str: &str = sliced_string.as_str();
    BigUint::from_str_radix(sliced_str, 16).unwrap().to_string()
}

pub fn fr_vec_to_leo_str(fr_vec: &[Fr]) -> String {
    let string_vec: Vec<String> = fr_vec.iter().map(|x| fr_to_leo_str(*x)).collect();
    format!("[{}]", string_vec.join(","))
}

/// Converts an Aleo Account to a Field
// Note the adresses of Testnet2/3 have a bigger set of elements than the fields of Testnet1, so there may be collisions
// TO DO: Investigate the issue
// Whenever Leo specifies how addresses should be used, change this function to fit the specification
pub fn aleo_account_str_to_fr(account_string: &str) -> Result<Fr, Box<dyn error::Error>> {
    let (_hrp, data, _variant) = bech32::decode(account_string)?;
    let base32_bytes: Vec<u8> = Vec::<u8>::from_base32(&data)?;
    let fr_string = BigUint::from_bytes_be(&base32_bytes).to_string();
    Ok(Fr::from_str(&fr_string).unwrap())
}

pub fn aleo_account_str_vec_to_fr_vec(
    accounts: Vec<String>,
) -> Result<Vec<Fr>, Box<dyn error::Error>> {
    accounts.iter().map(|x| aleo_account_str_to_fr(x)).collect()
}

pub fn aleo_account_str_to_leo_input(
    account_string: &str,
) -> Result<String, Box<dyn error::Error>> {
    Ok(fr_to_leo_str(aleo_account_str_to_fr(account_string)?))
}

#[cfg(test)]
mod tests {
    use super::aleo_account_str_to_fr;
    use ff::PrimeField;
    use poseidon_rs::Fr;

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

    #[test]
    fn aleo_account_to_fr() {
        let aleo_account_str = "aleo13j6lk3lvqjnymxjt3r080e226vt6933rv2e5dl2mt4226y5625qsumwz4z";
        println!(
            "{}",
            aleo_account_str_to_fr(aleo_account_str)
                .unwrap()
                .to_string()
        );
    }
}
