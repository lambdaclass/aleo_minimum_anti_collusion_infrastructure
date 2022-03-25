use rand::{rngs::StdRng, SeedableRng};
use snarkvm::{
    dpc,
    dpc::{testnet2::Testnet2, LedgerProof, Record},
    prelude::{Account, Address, Function, LedgerTree, Payload, PrivateKey, Transaction},
};

/// Creates a DPC request to store data in one record
/// To be used in a DPC Transaction
pub fn create_store_data_request(
    private_key: PrivateKey<Testnet2>,
    data: Vec<u8>,
    is_public: bool,
) -> dpc::Request<Testnet2> {
    let mut rng = StdRng::from_entropy();

    let noop_address = Address::from_private_key(&private_key);

    //Since we will use an insert function later
    //That uses only one register
    //We will input one payload, and use a dummy NOOP
    let payload_data = Payload::<Testnet2>::from(&data);
    let noop_record = Record::new_noop(noop_address, &mut rng).unwrap();

    //Remember: Testnet2 always have 2 input registers
    //And two outputs!
    let mut records = Vec::with_capacity(2);
    records.push(noop_record);

    let ledger_proof1 = LedgerProof::<Testnet2>::default();
    let ledger_proof2 = LedgerProof::<Testnet2>::default();
    let ledger_proofs = vec![ledger_proof1, ledger_proof2];

    let function_inputs = dpc::FunctionInputs::<Testnet2>::new(
        &noop_address,
        &noop_address,
        dpc::AleoAmount::ZERO,
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
        is_public,
        &mut rng,
    )
    .unwrap();
    return request;
}

/// Creates a transaction to store data in a register
pub fn create_store_data_in_event_transaction(data: Vec<u8>, is_public: bool) {
    let mut rand = StdRng::from_entropy();

    // The account should be generated in an upper layer
    let new_account = Account::<Testnet2>::new(&mut rand);
    let new_private_key = new_account.private_key();
    let account_view_key = new_account.view_key();

    let request = create_store_data_request(new_private_key.clone(), data, is_public);

    Transaction::new(LedgerTree::<Testnet2>::default(), &request, &mut rand).unwrap();
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};
    use serde::Serialize;
    use snarkvm::{
        dpc,
        dpc::{testnet2::Testnet2, Ledger, LedgerProof, VirtualMachine},
        prelude::{
            Account, Address, DecryptionKey, Event, PrivateKey, ToBytes, Transaction, ViewKey,
        },
    };

    use std::sync::atomic::AtomicBool;

    #[test]
    ///Creates a transaction and submits it to a local ledger
    /// This test lasts about 25 min
    fn local_transaction_test() {
        println!("Creating ledger ...");
        let mut ledger = Ledger::<Testnet2>::new().unwrap();
        println!("Done");
        println!("Creating proofs ...");
        let ledger_proof = LedgerProof::<Testnet2>::default();
        let ledger_proof2 = LedgerProof::<Testnet2>::default();
        println!("Done");
        let mut rand1 = StdRng::from_entropy();
        println!("Creating request ...");
        let request =
            dpc::Request::new_noop(vec![ledger_proof, ledger_proof2], &mut rand1).unwrap();
        println!("Done");
        let mut rand2 = StdRng::from_entropy();
        println!("Creating transaction ...");
        let transaction =
            Transaction::new(ledger.to_ledger_tree().clone(), &request, &mut rand2).unwrap();
        println!("Done");
        println!(
            "Transaction hexbyte: {}",
            hex::encode(transaction.to_bytes_le().unwrap())
        );
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
            .mine_next_block(new_account, false, &AtomicBool::new(false), &mut rand3)
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
        println!("Creating request ...");
        //This request is in fact, a transition
        let mut rng = StdRng::from_entropy();

        let new_account = Account::<Testnet2>::new(&mut rng);
        let new_private_key = new_account.private_key();
        let account_view_key = new_account.view_key();

        let request = create_store_data_request(new_private_key.clone(), vec![42; 128], false);
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
            .mine_next_block(new_account, false, &AtomicBool::new(false), &mut rand3)
            .unwrap();
        let x = ledger.latest_block().unwrap();
        let chain_transaction = x.transactions().last().unwrap();
        println!("Latest block: {}", x.to_string());
        println!("Block: {}", x.to_string());
        let decryption_key = DecryptionKey::from(account_view_key);
        let mut decrypted_records = chain_transaction.to_decrypted_records(&decryption_key);
        println!("Decrypted record 0: {}", decrypted_records.next().unwrap());
        println!("Decrypted record 1: {}", decrypted_records.next().unwrap());
    }

    #[test]
    fn asd() {
        let string_data = "57726974654F6E6C794C616273";
        println!("Str data: {:?}", string_data.as_bytes().to_vec());
    }
    #[test]
    fn local_transaction_with_public_data_test() {
        //This test lasts 25 min approx
        println!("Creating ledger ...");
        let mut ledger = Ledger::<Testnet2>::new().unwrap();
        println!("Done");
        println!("Creating request ...");
        //This request is in fact, a transition
        let mut rng = StdRng::from_entropy();

        let new_account = Account::<Testnet2>::new(&mut rng);
        let new_private_key = new_account.private_key();

        let string_data = "57726974654F6E6C794C616273";
        println!("Str data: {:?}", string_data.as_bytes().to_vec());
        let request = create_store_data_request(
            new_private_key.clone(),
            string_data.as_bytes().to_vec(),
            true,
        );
        println!("Done");
        let mut rand2 = StdRng::from_entropy();
        println!("Creating transaction ...");
        let transaction =
            Transaction::new(ledger.to_ledger_tree().clone(), &request, &mut rand2).unwrap();
        println!(
            "Transaction hexbyte: {}",
            hex::encode(transaction.to_bytes_le().unwrap())
        );
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
            .mine_next_block(new_account, false, &AtomicBool::new(false), &mut rand3)
            .unwrap();
        let x = ledger.latest_block().unwrap();
        let chain_transaction = x.transactions().last().unwrap();
        println!("Block: {}", x.to_string());
        println!("Transaction {}", chain_transaction.to_string());
    }

    /*

    This no longer works with SnarkVM 0.8.
    #[test]
    fn local_transaction_with_events_test() {
        //This test lasts 25 min approx
        println!("Creating ledger ...");
        let mut ledger = Ledger::<Testnet2>::new().unwrap();
        println!("Done");
        println!("Creating request ...");
        //This request is in fact, a transition
        let mut rng = StdRng::from_entropy();
        let new_account = Account::<Testnet2>::new(&mut rng);
        let new_private_key = new_account.private_key();
        let request = create_store_data_request(new_private_key.clone(), vec![42; 128], false);
        println!("Done");
        let mut rand2 = StdRng::from_entropy();
        println!("Creating transaction ...");

        let event = Event::Custom("Hola!".as_bytes().to_vec());
        let event2 = Event::Custom("Mundo!".as_bytes().to_vec());

        let transaction =
            VirtualMachine::<Testnet2>::new(LedgerProof::<Testnet2>::default().ledger_root())
                .unwrap()
                .add_event(event)
                .unwrap()
                .add_event(event2)
                .unwrap()
                .execute(&request, &mut rng)
                .unwrap()
                .finalize()
                .unwrap();
        println!("Done");
        println!("Transaction hexbyte: {}", hex::encode(transaction.to_bytes_le().unwrap()));
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
            chain_transaction.to_decrypted_records(new_account.view);
        println!("Decrypted record 0: {}", decrypted_records[0]);
        println!("Events 0: {}", chain_transaction.events()[0]);
        println!("Events 1: {}", chain_transaction.events()[1]);
    }*/
}

/*
curl --data-binary '{"jsonrpc": "2.0", "id":"1", "method": "gettransaction", "params": ["at1demecfadl03p9f9kax66s7jyf3x48asztkptcfxqgmkveymusgqsefxa7f"]}' -H 'content-type: application/json' http://localhost:3032/
at1demecfadl03p9f9kax66s7jyf3x48asztkptcfxqgmkveymusgqsefxa7f

curl --data-binary '{"jsonrpc": "2.0", "id":"1", "method": "gettransaction", "params": ["at1demecfadl03p9f9kax66s7jyf3x48asztkptcfxqgmkveymusgqsefxa7f"]}' -H 'content-type: application/json' http://localhost:3032/

8e20bb6d749540012794eecf23287736df7007e4161022733948bd38a8dfade919d4db332f220ba0561e9f7fcfed4d011b92d20746b44b8e5d489cb4135e6d012b55e2e5887bc6c3ee9b0d406071cc080100fac978c3e1cb71e3d6ba5fe644701f20442c2ed1dc822df59e93e6dc9eb018059465dca247f9a562ee3abbb819468051e8e5d29b178087eea5b8d2361c6ff90a8e4f281b7427a4b8b7ca5a59ada261e45158a944b5e84ec46b5e90823ab9330e21c2ce48ac0734109b9d73b998ad70353aeeb1b86455f62a226a4ad2a3cec5019df5ad8e7c7724a762d30b9eb015d5074975402cfc2ba79b9b1db4be64a26c107a8a7fc36fd937b998fd92dc5ecc8ab2cc8e3cb56bb417994e3f07521b5f80080518f631bfdf13146cd495442449b7492714801e2a14d80b65520a27cc3efd043fc8a8580421e209bada115168d4f2b4e577845f08ee9c5dd2076a57c6a9c803a72f144bb7000b45c16f7b1411665ff8c0b4fb02811e7b2b4ed131f52be1e704c7ec267a5e4c9f756ae5a4e84bf9af491b82474928bbc42909d4fa63c129d311b60df186ab10e585ef3916529a65f86576136927c363312a973e6459090e3a052cf341e857f4cb9242ca888f2be14b652a9f003704774fa93c174f6f0e74bf06d62e8570933b8a8fbbc54290becd749f8bb46e3d4dd3924fa269beb2c7d7b50c91b47af6b27f9d3ef8466ca636b837d44ad72e40ddf4e367deccfda215195009d23bc956baf9fe9b276ad06195819ffa020dfbc60b8557b0dc2c1b629b80fe0eba8f2615bc8bcbea9af5d1dacbf017a12f73a1dd17102d7ff083879260de5b09692858439fef02824119425a7c27c901ccccb9e50745dfe1e40cd2b854d47d0f2f017d5475c325cd899df27754a0c41d593e5f33796404993e7bfbbeda3e230adb83b38cef3e20c6982414913ea85c43d80cad712e60d7b2d9ef01ae69704b0641b29c04e28aa9c9b6cc572ea80a12ae172225112e9099042a893ace3c19e609115438f6560bdf4a4e1cfe9f0336d099e11e0f4c0a6058870c12dba18294ee07000000000000000002000203002df4f08453bdff2a229788f0093574a1c70bbf95213a7610f6c7bd7a21bb9547cf8e0cda704ae2be804e3ba2d27b560101f6a98b7ef65ecd2009f14e8e2475ce56ef02b33d16228d256a2779336b8eab00f6a98b7ef65ecd2009f14e8e2475ce56ef02b33d16228d256a2779336b8eab0000000000000000002a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0100631d5aca38b7c69941502687393885d9bacf4aefab04ef47aa5a52e27b475009c732136ef548e1226b06754a4280d3bb92e3a38e939148cc6931421d64a7d009fd125b6fe6ed8f259516c204b4cc49ad8f1d9e19422b164de6216664016b9308ee77deeb32b459dca107733142082b979dabf1fade3fa78f5a2a981fe4b87080760247ca862f96f3a8e7d14c066dedce6b16e075fcf5fc2c01f8e8b0e696a73e1959f390ead86ebe68643cfc6ee7920391d241ade08095e71b48dcd28a9d3c48f1e77d31b4e14459d66d3ba8bb1137e071d64be85d42f08c68fce85e94c2b180922e2b1ed57598f0412c95416c88274053dcbc74c2809e2a765db0b271a472fe361a33128711b2de3161f0ea81c2f13457110e34d18875317dbc2eed42e359341d1d93b2a5737e976ed1d9e4c75fe96482c312be5db6b928d954da77d770690001

 curl --data-binary '{"jsonrpc": "2.0", "id":"1", "method": "sendtransaction", "params": ["8e20bb6d749540012794eecf23287736df7007e4161022733948bd38a8dfade919d4db332f220ba0561e9f7fcfed4d011b92d20746b44b8e5d489cb4135e6d012b55e2e5887bc6c3ee9b0d406071cc0801001c892d4e8ca0684babbad7e25197d1c00effa7c4df8a826b259676c7c206ec016609b141c965672b530bcfc888dc14c7f5fb12234b239148c47d8529b78d9405cf36fd59075f97b04e89e95886744725c753041c238dcba478c9e544d149c509439c47032b0a509c45aead3a914304e878bdb48749f89c901d271b73e2bcfb0c78e72b45ba4e564c62d17e85b5b01f3222b06c8ed6f9dff8dbd1c2190664c60668b60eaf0af7290a90e92d804338f81b107e7176e400c6be5128f3a2df71ee10700825c3653ddc6139ff7e8e4961f3d9cc0a4e46ee194cdabd390f170ab3150e6c893d1f357e74b8ae5cd3bc9e7bacf9ddbbc652cff7c05a6c54739f9d49c10bd80c8a5599e0ed39610f1a239d26d8a012a3b43564202a23e89851f63dbcdd0bf150d34bda65439959c3d6e1db34fe43e35912d299d28c0f620e39cada85880b3bf8b2ddf8c6f6e1f9f53f6e0eab900a6972174a7ae150f16ad13a811487480f80eebfedaed1abad7e6ec60194018d97e93a54b4898010eca154b9391c84190ca7fe0c766c65bb8cd3b41de4140005f2b8105101b5ed8e384b80552ff763c507b4c96bc34f0d2125ceddad1567ad7ddf5be2015d63994e742c544c081024c00c9a11df213a8d525703403a9dfc359210abc63aadaf63855a19e10f448a696d0dc57fdb5ae231ce1311529bf348179179aa8bf9dd2e374f5ff27400f8368db705e5198038f3d1238246e03b26299ceab28ab967cc547c9eac9e72ccf25f020a048bd976d0e812b2fcc0f2e801ace5b05ec2d48449e844e8737d94971a80166e0c4a988dfddc67ddc02524457c36facd94796f15c148eef3d685b1571f6c261609248b04b495eeaa2776158f6adab0ea60f4c9ae67fbb9de5fa592b92ee17cef047fa48c628477fa074bb381cb8547a20e5f14824fd3967d587a8b67c471cde404000000000000000002000203002df4f08453bdff2a229788f0093574a1c70bbf95213a7610f6c7bd7a21bb9547cf8e0cda704ae2be804e3ba2d27b560101fc61fe805fe48672d28339b82dc402653464da292216cdc0e8682f52e0f50706fc61fe805fe48672d28339b82dc402653464da292216cdc0e8682f52e0f50706000000000000000035373732363937343635344636453643373934433631363237330000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100a50e0f680513f6ac64e8009cfd2b59bbe71ba076088ffea4bfd5054587464f0e7a0069b08c7ccc132b80156f082a728db8967bb1e74334100e03ea8dc6f8ceda01ecbcdbe161bc6ee5a9f578da23c790ba733bbde8c949b1d85d2e182f87765f2beac887b09bee47bc19e5dbfec65aa89edba081c2983c4c6b2e878ea1785c00e3543afb7abd7c80b2703a1c9cf21b3e8920665f68e15baec91127932f869432025691408625a7d6434867a8b4c51c37031d9be41d9d1926c2c1aef3969daf9d354f2edbc71a738e914ecbab932172014c043b6df84fdd58f9ad5d2a4baf25001f021f74c855ca137b0442c4bc5ee3d47b8d78504faeee89f87d48d66b7106e6fb75ecf1bf99b74324b4990ab6ed22ced13ddb4c0ae496756357f372c92ea1822e21bf8cee292321aedd7c9fc740d762576bf6f986cfe8bc4a0732f741bd070001"]}' -H 'content-type: application/json' http://188.166.7.13:3032/
*/
