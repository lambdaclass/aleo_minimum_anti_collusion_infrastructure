// Note: this requires the `derive` feature
use aleo_maci_libs::{aleo_account::account_utils, rcp, transactions};
use clap::{Parser, Subcommand};
use serde_json::{json, Value};
use snarkvm::{
    dpc::testnet2::Testnet2,
    prelude::{Account, PrivateKey, ToBytes},
};
use std::str::FromStr;
#[derive(Parser)]
#[clap(name = "aleo-maci-cli")]
#[clap(about = "A CLI to use MACI in Aleo's blockchain", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Stores a vote for the given option in the blockchain
    #[clap(arg_required_else_help = true)]
    VoteFor {
        /// Vote for the given option, must be a number between 1 and the max amount of options
        message_data: u8,
        /// Account private key to be used to send the vote
        account_private_key: String,
    },
    /// Creates a new Aleo account, and outputs its private key
    CreateAleoAccount {},
    /// Helper function to retrieve the address of an account from its private key
    #[clap(arg_required_else_help = true)]
    AddressOf {
        /// Account private key
        account_private_key: String,
    },
}
fn main() {
    let args: Cli = Cli::parse();

    //TO DO: Add the logic to the commands
    match &args.command {
        Commands::VoteFor {
            message_data,
            account_private_key,
        } => {
            println!("Generating the transaction...");
            println!("This may take a while");

            let private_key_result = PrivateKey::<Testnet2>::from_str(account_private_key);

            let private_key = match private_key_result {
                Ok(value) => value,
                Err(_) => {
                    eprintln!("Account private key is not valid");
                    return;
                }
            };

            let account: Account<Testnet2> = Account::<Testnet2>::from(private_key);

            let transaction_payload: Vec<u8> = vec![*message_data];
            let transaction =
                transactions::create_store_data_transaction(transaction_payload, account, true);
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
                eprintln!("Aleo nodes can't be reached, try again later");
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
                    eprintln!("Election server can't be reached, try again later")
                }
            }

            //TO DO: Add a command to check the block has been mined after a while
            //And retry without generating the transaction later
        }

        Commands::CreateAleoAccount {} => {
            let new_account = account_utils::create_new_account();
            println!("Your new account private key is");
            println!("{}", new_account.private_key());
            println!("Make sure to store it in a safe place");
        }

        Commands::AddressOf {
            account_private_key,
        } => {
            let private_key_result = PrivateKey::<Testnet2>::from_str(account_private_key);

            let private_key = match private_key_result {
                Ok(value) => value,
                Err(_) => {
                    eprintln!("Account private key is not valid");
                    return;
                }
            };

            let address = Account::<Testnet2>::from(private_key).address();

            println!("{}", address);
        }
    }
}
