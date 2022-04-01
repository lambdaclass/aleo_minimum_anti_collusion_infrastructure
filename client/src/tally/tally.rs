mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};
    use snarkvm::{
        algorithms,
        curves::{bls12_377::Bls12_377Parameters, templates::bls12::Bls12},
        dpc,
        dpc::{
            testnet1::Testnet1, testnet2::Testnet2, Ledger, LedgerProof, Network, VirtualMachine,
        },
        prelude::{
            Account, Address, AleoAmount, DecryptionKey, FromBytes, Function, FunctionInputs,
            Operation, Payload, PrivateKey, Record, Request, ToBytes, Transaction, ViewKey,
        },
    };

    use std::sync::atomic::AtomicBool;

    #[test]
    fn test_circuit_snarkvm() {
        //Note until Leo 2.0, we use Testnet1 as to use Groth16 instead of Marlin
        let vm = VirtualMachine::<Testnet1>::new(LedgerProof::<Testnet1>::default().ledger_root())
            .unwrap();

        let vk_bytes = include_bytes!("../../../circuits/tally/outputs/tally.lvk");
        let pk_bytes = include_bytes!("../../../circuits/tally/outputs/tally.lpk");
        let vk = algorithms::snark::groth16::VerifyingKey::<Bls12<Bls12_377Parameters>>::read_le(
            vk_bytes.as_slice(),
        )
        .unwrap();
        let pk = algorithms::snark::groth16::ProvingKey::<Bls12<Bls12_377Parameters>>::read_le(
            pk_bytes.as_slice(),
        )
        .unwrap();
        println!("Vk: {:?}", vk);
        println!("Pk: {:?}", pk);

        let mut rand = StdRng::from_entropy();
        let private_key = &PrivateKey::<Testnet1>::new(&mut rand);
        let new_account: Address<Testnet1> =
            Address::from_private_key(&PrivateKey::<Testnet1>::new(&mut rand));

        let function_id = <Testnet1 as Network>::function_id(&vk).unwrap();
        let function_inputs = FunctionInputs::<Testnet1>::new(
            &new_account,
            &new_account,
            AleoAmount::ZERO,
            Payload::default(),
        );
        let operation =
            Operation::Evaluate(function_id, dpc::FunctionType::Insert, function_inputs);
        let request = Request::<Testnet1>::new(
            private_key,
            vec![Record::default(), Record::default()],
            vec![LedgerProof::default(), LedgerProof::default()],
            operation,
            AleoAmount::ZERO,
            true,
            &mut rand,
        )
        .unwrap();

        // let function = Function
        // vm.execute_program(&request, program_id, function, function_path, vk, private_variables, custom_events, rng);
    }
}
