use ff::{Field, PrimeField};
use mimc_rs::{Fr, Mimc7};

fn encrypt(mut message: Fr, shared_key: Fr) -> Fr {
    //TO DO: Generate a proper IV (Initialization Value)
    //Note:: This work as a salt
    let initialization_vector = Fr::from_str("0").unwrap();
    let mimc7 = Mimc7::new(91);
    let key_hash = mimc7.hash(&shared_key, &initialization_vector);
    message.add_assign(&key_hash);
    message
}

fn decrypt(mut message: Fr, shared_key: Fr) -> Fr {
    let initialization_vector = Fr::from_str("0").unwrap();
    let mimc7 = Mimc7::new(91);
    let key_hash = mimc7.hash(&shared_key, &initialization_vector);
    message.sub_assign(&key_hash);
    message
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let message = Fr::from_str("15").unwrap();
        let shared_key = Fr::from_str("42").unwrap();
        assert_eq!(
            decrypt(encrypt(message, shared_key), shared_key).to_string(),
            "Fr(0x000000000000000000000000000000000000000000000000000000000000000f)"
        );
    }

    #[test]
    fn test_encrypt_decrypt_big_number() {
        //7444461749428370424248824938781546531375899335154063827935233455917409239041
        // == 0x10756a81186a232aacc3d4a6c4fa5f0ecdb84ccd117118010a11800000000001
        let message = Fr::from_str(
            "7444461749428370424248824938781546531375899335154063827935233455917409239041",
        )
        .unwrap();
        let shared_key = Fr::from_str(
            "6444461749428370424248824938781546531375899335154063827935233455917409239041",
        )
        .unwrap();
        assert_eq!(
            decrypt(encrypt(message, shared_key), shared_key).to_string(),
            "Fr(0x10756a81186a232aacc3d4a6c4fa5f0ecdb84ccd117118010a11800000000001)"
        );
    }
}
