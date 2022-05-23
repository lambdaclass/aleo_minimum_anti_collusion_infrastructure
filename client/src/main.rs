// Note: this requires the `derive` feature
use aleo_maci_libs::{rcp, transactions};
use clap::{Parser, Subcommand};
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
    /// Sign Ups to an election
    #[clap(arg_required_else_help = true)]
    SignUp {
        /// Public key to use. To generate one, call generate-key-pair
        public_key: String,
        /// Election id
        election_id: String,
    },
    /// Stores a vote for the given option in the blockchain
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
        Commands::SignUp {
            public_key: _,
            election_id: _,
        } => {
            println!("Signing up ...");
        }

        Commands::VoteFor { message_data } => {
            println!("Generating the transaction...");
            println!("This may take a while");

            //TO DO: Let the user make an account and use it, instead
            //of creating it with a random one
            let transaction_payload: Vec<u8> = vec![*message_data];
            let transaction =
                transactions::create_store_data_transaction(transaction_payload, true);
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
                        amount_of_bad_results += 1;
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
