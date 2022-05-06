// Note: this requires the `derive` feature
use aleo_maci_libs::{
    crypto,
    crypto::{Fr, PrimeField},
    rcp, transactions,
};
use clap::{Parser, Subcommand};
use num::{BigUint, Num};
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
    /// [FOR TEST] stores data in the blockchain
    #[clap(arg_required_else_help = true)]
    VoteFor {
        /// Vote for the given option, must be a number between 1 and the max amount of options
        message_data: u8,
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
        Commands::VoteFor { message_data } => {
            println!("Generating the transaction...");
            println!("This may take a while");

            let shared_key: Fr = Fr::from_str("42").unwrap();
            let fr_data = Fr::from_str(&message_data.to_string()).unwrap();
            let encrypted_data_str = crypto::cipher::encrypt(fr_data, shared_key).to_string();
            //TO DO: Let the user make an account and use it, instead
            //of creating it with a random one
            let encrypted_data_str =
                encrypted_data_str[5..(encrypted_data_str.len() - 1)].to_string();
            let encrypted_data_big_uint = BigUint::from_str_radix(&encrypted_data_str, 16).unwrap();

            let transaction = transactions::create_store_data_transaction(
                encrypted_data_big_uint.to_bytes_be(),
                true,
            );

            //TO DO: Use the real shared key
            let encoded_data = hex::encode(transaction.to_bytes_le().unwrap());
            println!("The transaction hexdata is: \n {}", encoded_data);
            println!("Sending transactions to multiple nodes ...");
            // To improve reliability we send the transaction to many nodes
            let responses = rcp::sync_spray_transaction(encoded_data);

            let mut ok_response: Value = json!("");
            let mut amount_of_bad_results = 0;
            for response in responses.iter() {
                match response {
                    Ok(value) => {
                        ok_response = value.clone();
                        break;
                    }
                    Err(_) => {
                        amount_of_bad_results = amount_of_bad_results + 1;
                    }
                }
            }

            if responses.len() == amount_of_bad_results {
                println!("Aleo nodes can't be reached, try again later");
                return;
            }

            println!("Nodes received the transaction succesfully");

            let transaction_id = ok_response.get("result").unwrap();

            println!("The transaction id is: {}", transaction_id);

            println!("Notifying the transaction submission to the tallying server ...");

            let request_json = json!({ "aleo_transaction_id": transaction_id });

            let client = reqwest::blocking::Client::new();
            let send_result = client
                .post("http://127.0.0.1:3000/election/msg")
                .json(&request_json)
                .send();

            match send_result {
                Ok(_) => {
                    println!("Vote process finished successfully")
                }
                Err(_) => {
                    println!("Election server can't be reached, try again later")
                }
            }

            //TO DO: Add a command to check the block has been mined after a while
            //And retry without generating the transaction later
        }
    }
}
