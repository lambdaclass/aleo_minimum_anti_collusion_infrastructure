mod transactions;
// Note: this requires the `derive` feature
use clap::{Parser, Subcommand};
use ring::{
    rand,
    signature::{self, KeyPair},
};
use serde_json::{json, Value};
use snarkvm::prelude::ToBytes;
#[derive(Parser)]
#[clap(name = "aleo-maci-cli")]
#[clap(about = "A CLI to use MACI in Aleo's blockchain", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a key pair to use with other commands
    GenerateKeyPair {},
    /// Sign Ups to an election
    #[clap(arg_required_else_help = true)]
    SignUp {
        /// Public key to use. To generate one, call generate-key-pair
        public_key: String,
        /// Election id
        election_id: String,
    },
    /// Publish a message, which can be a vote, a change of a public key, or both
    // This should also get the user private key
    // Current_pk may be changed for the full pair for ease of use
    #[clap(arg_required_else_help = true)]
    Publish {
        /// Current public key
        current_pk: String,
        //new_pk should be current_pk by the default
        /// New public key. Can be the same as current one
        new_pk: String,
        /// Number of option to vote
        vote_option: u8,
        /// Election id
        election_id: String,
    },
    /// Creates a new election
    #[clap(arg_required_else_help = true)]
    CreateElection {
        /// Sign up duration in seconds
        sign_up_duration: u32,
        /// Voting duration in seconds
        voting_duration: u32,
    },
    /// Starts tallying the vote in the server
    #[clap(arg_required_else_help = true)]
    StartTally {
        /// Election id
        election_id: String,
    },
    /// Generates a key pair to use with other commands
    #[clap(arg_required_else_help = true)]
    StoreMessage {
        /// Up to 128 bytes of data as a string
        message_data: String,
    },
}
fn main() {
    let args: Cli = Cli::parse();

    //TO DO: Add the logic to the commands
    match &args.command {
        Commands::GenerateKeyPair {} => {
            println!("Generating key pair ...");
            // Generate a key pair in PKCS#8 (v2) format.
            let rng = rand::SystemRandom::new();
            let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

            // Normally the application would store the PKCS#8 file persistently. Later
            // it would read the PKCS#8 file from persistent storage to use it.

            let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
            let public_key = key_pair.public_key().as_ref();

            println!("Your public key is:\n 0x{}", hex::encode(public_key));
            //Note: It's using PKCS 8 v2, so it's priv + pub
            println!(
                "Your key pair is (priv + pub, PKCS 8 v2) is :\n 0x{}",
                hex::encode(pkcs8_bytes.as_ref())
            );
        }
        Commands::SignUp {
            public_key: _,
            election_id: _,
        } => {
            println!("Signing up ...");
        }

        Commands::Publish {
            current_pk: _,
            new_pk: _,
            vote_option,
            election_id: _,
        } => {
            println!("Publishing vote for {} ...", vote_option);
        }
        Commands::CreateElection {
            sign_up_duration: _,
            voting_duration: _,
        } => {
            println!("Creating election ...");
        }
        Commands::StartTally { election_id: _ } => {
            println!("Starting tally ...");
        }
        Commands::StoreMessage { message_data } => {
            println!("Generating the transaction...");
            println!("This may take a while");

            let transaction = transactions::create_store_data_transaction(
                message_data.as_bytes().to_vec(),
                true,
            );
            println!("Submitting the transaction to the blockchain");

            let encoded_data = hex::encode(transaction.to_bytes_le().unwrap());

            //let encoded_data = "a8a2358b0aa49123434b1757172e034d119cc282cfceb42210f886920ae6754d043539a82da297d5578504fcab0033007fd1e99cb0aadde4134e27d027f03220fda23f7b953a2e803f9a636b06d0990701004b603407b56f96009ce85a8dd2499de53e769450134f34da69f14df188f5fe0e0407f023b9ee4936be5f456d7cfd034f300029bbbc898326c9651bf5eac16e0a112f1518f00604d5a1a07d69b861be24778143f51513753e59d6d5f64668750471a6179c699931426cc71e0cecf5057bc5e46f5ce5657baf287cc0b0fca4db05a3066575b37247db16e98e83d53ef33834438dcf1bc57d903bcddc2bea4f190d3c59de1c022b7b575241f266bee6ca0925c6fbf4e0b63f02eb123ce38f1c320f90cc2b3a82dfb9e2f295acb9e4c5c977fccbc30db9de708aa4ece48f12a5ad06fcb42ca8a755898c4d9e1cc8394a416d3b73edcce9774e808d1b6bc6046dc10a5b2518077a59dbafc7f9b9ac81cbde772fd008b155a83f14f0846b88e02d5309d919e6313d183f3bf0af83cbd76119517e4e287835eee9369cba022c2572ce08c2d72c2625c59a1b2025354ec990b9776da9a2b20f4b9915bff5e32e4c66390b1439b13f762baa16d7da7828b7cbd548177e08344d534a4e42a749a2c1263e03de9b0fd96505e8f279aa916cd3b5dbd540abcc75a6f1f8e0bb785616b00861029a9f77559c5a0909b21d6055feae170e3b9b1c2716c8f9b00ad13a468737d3019abab4273892dd8ecaa22f12b05eaaf98bb55326472dfd01b2fcb4503f2ca70e8a8c184abb080c7ee316d238d343457750102b27e93629ccc65a622b665a3206d678f6f71dc1f2e57d7cb9e92fec0c3acc0e5cecd78b75ceffddc21f4719df07803544fe89482519660d4c6252695d72fc667a0dfe767ad91e75d92db907e4098bf667977db69436fb9be670a22848b471336e7c8ef28474e7e0db5a273d58057752e1eeed5d8d70f0d92ec05dbff12e11c6dd3dfad2f1a58818e27f35059404a52e6fdc636a2030808addb8a9c8acc0b604a5974f5a8a75e40f7577ac2d0700cf33741eff22fe00a6f2c54740329c1feefdc0f166a8206eef6a38b7e33968085e517840c2b6474ef5d11ef9676eb40af1e2136db1511257550cc8999159940ab6fd45ce3cfbe1772da98d562b7aea671ec9d6090156ea32ef10993b8efb7f120000000000000000f71f34e558f33b41156f8548761b67fe44847233993aad392b9cd78083d73858cf41b4f5594b187d51c37d361137732f37049f251735e464720d5b708ab3a34af440c5b8c40cee739e08b0218a86339f3bd83e1536830c9da7a673e357340c001a7a0a32c60418d5432858dea675277eb7f1e37943b233025b6a37045f66a09900a820bd572769e8ce866f4eb2fe1855cf1d365b73c15bd847b39579e5cfa4d4622c04f80d0b2a7680455e7ca507d49597352bd3b0de7480ccfbd6d44e274d805bd82c0eead22bfb79f9e891564ec9e827d9abd7df04d4f2fd0bf6284aae4609be4fc4879a0d44fef39203b14d83be36455a293464a1d893821dd77352a942c69d0cfd9cc4940a2278de1a1ca968d2fc37fdee803aec499279c17b322b52de8001010002030041bec10d7cba19aecd9969ab90aafb6ee3f415c8a57f37334964bfe437f9b0517ab148ad900a7b1436a41d67a8d9a00101324266f5f7be215a54d4add7fae5486cf07b9665819c6b5359c1ec697ec0930d324266f5f7be215a54d4add7fae5486cf07b9665819c6b5359c1ec697ec0930d00000000000000002a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a";

            let request_json = json!({
                "jsonrpc": "2.0",
                "id": "1",
                "method": "sendtransaction",
                "params": [
                    encoded_data
                ]
            });

            let client = reqwest::blocking::Client::new();

            //TO DO: Use our own client
            let res = client
                .post("http://144.126.212.176:3032/")
                .json(&request_json)
                .send()
                .unwrap();

            let response_json: Value = res.json().unwrap();

            println!(
                "Your transaction id is: \n{}",
                response_json["result"].as_str().unwrap()
            );
            println!("Store it to use it later");
        }
    }
}
