use rand::{rngs::StdRng, SeedableRng};
use snarkvm::{dpc::testnet2::Testnet2, prelude::Account};

pub fn create_new_account() -> Account<Testnet2> {
    Account::<Testnet2>::new(&mut StdRng::from_entropy())
}
