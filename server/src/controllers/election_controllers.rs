use crate::models::{Election, Tally};
use crate::r2d2::Pool;
use crate::services::leo_io::generate_input_file;
use crate::utils::votes_to_fix_array;
use crate::RedisConnectionManager;
use aleo_maci_libs::{rcp::encrypted_public_data_to_vote, rcp::get_transaction_public_data};

use r2d2_redis::redis::Commands;
use serde::Deserialize;
use serde_json::json;
use std::process::Command;
use warp::reply::Json;

#[derive(Debug, Deserialize)]
pub struct ElectionCreate {
    sign_up_duration: i64,
    voting_duration: i64,
}

#[derive(Debug, Deserialize)]
pub struct ElectionSignUp {}

#[derive(Debug, Deserialize)]
pub struct ElectionMsg {
    aleo_transaction_id: String,
}

pub async fn create(
    data: ElectionCreate,
    _pool: Pool<RedisConnectionManager>,
) -> Result<Json, warp::Rejection> {
    let election = Election::new(data.sign_up_duration, data.voting_duration);

    Ok(warp::reply::json(&election))
}

pub async fn sign_up(
    _data: ElectionSignUp,
    _pool: Pool<RedisConnectionManager>,
) -> Result<Json, warp::Rejection> {
    Ok(warp::reply::json(&json!({"msg":"not implemented"})))
}

pub async fn store_msg(
    data: ElectionMsg,
    pool: Pool<RedisConnectionManager>,
) -> Result<Json, warp::Rejection> {
    //TO DO: Don't store repeated values
    let mut con = match pool.get() {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    let _: () = con.lpush("votes", &data.aleo_transaction_id).unwrap();

    Ok(warp::reply::json(
        &json!({"msg":"your vote was succesuffly stored"}),
    ))
}

#[derive(Debug)]
struct DBError;

impl warp::reject::Reject for DBError {}

pub async fn start_tally(pool: Pool<RedisConnectionManager>) -> Result<Json, warp::Rejection> {
    // get redis pool connection
    let mut con = match pool.get() {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    // get transaction id from redis db
    let votes_ids_from_pool: Vec<String> = match con.lrange("votes", 0, -1) {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    println!(
        "Aleo Transactions to be computed: {:?}",
        votes_ids_from_pool
    );

    // Get votes from the Aleo Ledger
    let mut votes: Vec<String> = Vec::new();
    for v in votes_ids_from_pool {
        let public_data = get_transaction_public_data(v.to_string()).await;
        let vote = encrypted_public_data_to_vote(public_data.unwrap());
        votes.push(vote);
    }

    println!("Votes to be counted: {:?} ", votes);

    println!("Doing the tally...");

    let tally = Tally::new(
        votes
            .clone()
            .into_iter()
            .map(|v| v.parse::<u32>().unwrap())
            .collect(),
    );

    println!("Tally finished: {:?}", tally);

    // Generate circuit input file
    println!("Generating circuit input...");
    generate_input_file(votes_to_fix_array(&tally.votes), &tally.votes_markle_root);

    // Run circuit
    // TO DO: Make async, don't run if it's already running or has run before
    println!("Running circuit to verify the tally. Please wait a minute...");
    Command::new("sh")
        .arg("-c")
        .arg("make run_circuits")
        .output()
        .expect("failed to execute process");

    println!("Circuit execution finished.");

    Ok(warp::reply::json(&tally))
}
