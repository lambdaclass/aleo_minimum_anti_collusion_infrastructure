// Note: this requires the `derive` feature

use clap::{Parser, Subcommand};
#[derive(Parser)]
#[clap(name = "maci-cli")]
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
}

fn main() {
    let args: Cli = Cli::parse();

    //TO DO: Add the logic to the commands
    match &args.command {
        Commands::GenerateKeyPair {} => {
            println!("Generating key pair ...");
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
    }
}
