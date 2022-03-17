#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};
    use snarkvm::{
        dpc,
        dpc::{testnet2::Testnet2, Ledger, LedgerProof, Record},
        prelude::{Address, Function, LedgerTree, Payload, PrivateKey, Transaction, ViewKey},
        traits::LedgerTreeScheme,
        utilities::ToBytes,
    };

    use std::sync::atomic::AtomicBool;

    #[test]
    fn local_transaction_test() {
        //This test lasts 25 min approx
        println!("Creating ledger ...");
        let mut ledger = Ledger::<Testnet2>::new().unwrap();
        println!("Done");
        //Here we have to generate a transaction
        //Here we have to mine the transaction
        println!("Creating proofs ...");
        let ledger_proof = LedgerProof::<Testnet2>::default();
        let ledger_proof2 = LedgerProof::<Testnet2>::default();
        println!("Done");
        //let virtual_machine = VirtualMachine::<Testnet2>::new(ledger_proof.ledger_root());
        let mut rand1 = StdRng::from_entropy();
        println!("Creating request ...");
        //This request is in fact, a transition
        let request =
            dpc::Request::new_noop(vec![ledger_proof, ledger_proof2], &mut rand1).unwrap();
        println!("Done");
        let mut rand2 = StdRng::from_entropy();
        println!("Creating transaction ...");
        let transaction =
            Transaction::new(ledger.to_ledger_tree().clone(), &request, &mut rand2).unwrap();
        println!("Done");
        println!("Adding unconfirmed transaction ...");
        ledger.add_unconfirmed_transaction(&transaction).unwrap();
        println!("Done");
        let mut rand64 = StdRng::from_entropy();
        let new_account: Address<Testnet2> =
            Address::from_private_key(&PrivateKey::<Testnet2>::new(&mut rand64));
        let mut rand3 = StdRng::from_entropy();
        let y = ledger.latest_block().unwrap();
        println!("Latest block before transaction: {}", y.to_string());
        ledger
            .mine_next_block(new_account, &AtomicBool::new(false), &mut rand3)
            .unwrap();
        let x = ledger.latest_block().unwrap();
        println!("Latest block: {}", x.to_string());
        println!("Transaction: {}", transaction.to_string());
        println!("Block: {}", x.to_string());
    }

    #[test]

    fn local_transaction_with_data_test() {
        //This test lasts 25 min approx
        println!("Creating ledger ...");
        let mut ledger = Ledger::<Testnet2>::new().unwrap();
        println!("Done");
        let mut rand1 = StdRng::from_entropy();
        println!("Creating request ...");
        //This request is in fact, a transition
        let mut rng = StdRng::from_entropy();
        let new_private_key = PrivateKey::new(&mut rng);
        let request = create_store_data_request_noop_dummy(new_private_key.clone());
        println!("Done");
        let mut rand2 = StdRng::from_entropy();
        println!("Creating transaction ...");
        let transaction =
            Transaction::new(ledger.to_ledger_tree().clone(), &request, &mut rand2).unwrap();
        println!("Done");
        println!("Adding unconfirmed transaction ...");
        ledger.add_unconfirmed_transaction(&transaction).unwrap();
        println!("Done");
        let mut rand64 = StdRng::from_entropy();
        let new_account: Address<Testnet2> =
            Address::from_private_key(&PrivateKey::<Testnet2>::new(&mut rand64));
        let mut rand3 = StdRng::from_entropy();
        let y = ledger.latest_block().unwrap();
        println!("Latest block before transaction: {}", y.to_string());
        ledger
            .mine_next_block(new_account, &AtomicBool::new(false), &mut rand3)
            .unwrap();
        let x = ledger.latest_block().unwrap();
        let chain_transaction = x.transactions().last().unwrap();
        println!("Latest block: {}", x.to_string());
        println!("Block: {}", x.to_string());
        let decrypted_records =
            chain_transaction.to_decrypted_records(&ViewKey::from_private_key(&new_private_key));
        println!("Decrypted record 0: {}", decrypted_records[0]);
        println!("Decrypted record 1: {}", decrypted_records[1]);
    }

    //Note this doesn't work, inner circuit fails when the payload
    //of a dummy is not 0.
    fn create_store_data_request_noop_dummy(
        private_key: PrivateKey<Testnet2>,
    ) -> dpc::Request<Testnet2> {
        let mut rng = StdRng::from_entropy();

        let noop_address = Address::from_private_key(&private_key);

        //Remember 2: Payload size in testnet2
        //const RECORD_PAYLOAD_SIZE_IN_BYTES: usize = 128;
        let byte_data: Vec<u8> = vec![42; 128];

        //Since we will use an insert function later
        //We will only use one payload data,
        //The other is a noop
        let payload_data = Payload::<Testnet2>::from(&byte_data);

        let noop_record = Record::new_noop_input(noop_address, &mut rng).unwrap();

        //Remember: Testnet2 always have 2 input registers
        //And two outputs!
        let mut records = Vec::with_capacity(2);

        records.push(noop_record);

        let ledger_proof1 = LedgerProof::<Testnet2>::default();
        let ledger_proof2 = LedgerProof::<Testnet2>::default();
        let ledger_proofs = vec![ledger_proof1, ledger_proof2];

        let amount = dpc::AleoAmount::ZERO;
        let function_inputs = dpc::FunctionInputs::<Testnet2>::new(
            &noop_address,
            &noop_address,
            amount,
            payload_data,
        );

        let operation = dpc::Operation::<Testnet2>::Evaluate(
            dpc::virtual_machine::Noop::<Testnet2>::new().function_id(),
            dpc::FunctionType::Insert,
            function_inputs,
        );

        let request = dpc::Request::<Testnet2>::new(
            &private_key,
            records,
            ledger_proofs,
            operation,
            dpc::AleoAmount::ZERO,
            false,
            &mut rng,
        )
        .unwrap();

        return request;
    }

    //WIP
    fn rcp_transaction_test() {
        //This test lasts 25 min approx

        // Initialize a new ledger.
        const GET_LEDGER_PROOF_METHOD_NAME: &str = "getledgerproof";

        println!("Done");
        println!("Creating proofs ...");
        let ledger_proof = LedgerProof::<Testnet2>::default();
        let ledger_proof2 = LedgerProof::<Testnet2>::default();
        println!("Done");
        //let virtual_machine = VirtualMachine::<Testnet2>::new(ledger_proof.ledger_root());
        let mut rand1 = StdRng::from_entropy();
        println!("Creating request ...");
        //This is in fact a transaction
        let request =
            dpc::Request::new_noop(vec![ledger_proof, ledger_proof2], &mut rand1).unwrap();

        println!("Done");
        let mut rand2 = StdRng::from_entropy();
        println!("Creating transaction ...");

        //Here we need a ledger proof
        //The noop operation consumes a record, and
        let transaction =
            Transaction::new(LedgerTree::<Testnet2>::new().unwrap(), &request, &mut rand2).unwrap();
        println!("Done");
        println!("Transaction: {}", transaction);

        let transaction_bytes = hex::encode(transaction.to_bytes_le().unwrap());
        /*
        println!("Done");
        println!("Adding unconfirmed transactoin ...");
        ledger.add_unconfirmed_transaction(&transaction).unwrap();
        println!("Done");
        let mut rand64 = StdRng::from_entropy();
        let new_account: Address<Testnet2> =
            Address::from_private_key(&PrivateKey::<Testnet2>::new(&mut rand64));
        let mut rand3 = StdRng::from_entropy();
        let y = ledger.latest_block().unwrap();
        println!("Latest block before transaction: {}", y.to_string());
        ledger
            .mine_next_block(new_account, &AtomicBool::new(false), &mut rand3)
            .unwrap();
        let x = ledger.latest_block().unwrap();
        println!("Latest block: {}", x.to_string());
        println!("Transaction: {}", transaction.to_string());
        println!("Block: {}", x.to_string());*/
    }
}