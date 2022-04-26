use crate::r2d2::Pool;
use crate::services::leo_io::generate_input_file;
use crate::RedisConnectionManager;
use crate::{models::Election, services::leo_io};
use aleo_maci_libs::{
    merkle_tree::generate_merkle_root, rcp::get_transaction_public_data, rcp::public_data_to_vote,
};
use ff::PrimeField;
use poseidon_rs::Fr;

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
pub struct ElectionSignUp {
    public_key: String,
}

#[derive(Debug, Deserialize)]
pub struct ElectionMsg {
    aleo_transaction_id: String,
}
pub struct StartTally {
    tally_id: String,
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
    pool: Pool<RedisConnectionManager>,
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
    const MAX_AMOUNT_OF_VOTES: isize = 32;

    // get redis pool connection
    let mut con = match pool.get() {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    // get transaction id from redis db
    let votes_ids_from_pool: Vec<String> = match con.lrange("votes", 0, MAX_AMOUNT_OF_VOTES - 1) {
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
        let vote = public_data_to_vote(public_data.unwrap());
        votes.push(vote);
    }

    println!("Votes: {:?} ", votes);
    // Generate markle root of the votes for the circuit input
    let votes_merkle_root_fr_str = generate_merkle_root(
        votes
            .iter()
            .map(|v| Fr::from_str(&v.to_string()).unwrap())
            .collect(),
    )
    .to_string();

    let votes_merkle_root_leo_str = leo_io::fr_string_to_leo_str(votes_merkle_root_fr_str);

    // Transform votes to a fixed array of MAX_AMOUNT_OF_VOTES elements
    // if there is and invalid vote option, it will be count as 0
    // if there is less than MAX_AMOUNT_OF_VOTES, the reamaning votes will be count as 0
    let mut votes_fixed: [u32; MAX_AMOUNT_OF_VOTES as usize] = [0; MAX_AMOUNT_OF_VOTES as usize];
    for i in 0..MAX_AMOUNT_OF_VOTES {
        votes_fixed[i as usize] = match votes.get(i as usize) {
            Some(v) => match v.parse::<u32>() {
                Ok(v) => v,
                Err(_) => 0,
            },
            None => 0,
        };
    }

    // Generate circuit input file
    generate_input_file(votes_fixed, votes_merkle_root_leo_str.as_str());

    // Run circuit
    //TO DO: Make async, don't run if it's already running or has run before
    Command::new("sh")
        .arg("-c")
        .arg("make run_circuits")
        .output()
        .expect("failed to execute process");

    Ok(warp::reply::json(&json!({"msg":"Tally has begun"})))
}
