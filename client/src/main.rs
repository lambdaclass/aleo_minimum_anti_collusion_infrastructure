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
    /// As of this version only the vote option matters
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

            //TO DO: Let the user make an account and use it, instead
            //of creating it with a random one
            let transaction =
                transactions::create_store_data_transaction(message_data.as_bytes().to_vec(), true);
            let encoded_data = hex::encode(transaction.to_bytes_le().unwrap());
            //TO DO: Persist the encoded_data to retry the submission
            println!("Your transaction hexdata is:\n {}", encoded_data);
            println!("Submitting the transaction to the blockchain");

            //TO DO: Move all this Request logic to a shared library
            let request_json = json!({
                "jsonrpc": "2.0",
                "id": "1",
                "method": "sendtransaction",
                "params": [
                    encoded_data
                ]
            });
            let client = reqwest::blocking::Client::new();
            //TO DO: Use our own Aleo node client, or pass it as an argument
            let res = client
                .post("http://188.166.7.13:3032/")
                .json(&request_json)
                .send()
                .unwrap();
            let response_json: Value = res.json().unwrap();
            println!("The node response is: {}", response_json);
            //TO DO: Have a retry logic
        }
    }
}
