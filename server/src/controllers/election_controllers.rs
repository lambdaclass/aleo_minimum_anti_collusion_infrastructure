use crate::r2d2::Pool;
use crate::services::leo_io::generate_input_file;
use crate::RedisConnectionManager;
use crate::{models::Election, services::leo_io};
use aleo_maci_libs::merkle_tree::generate_merkle_root;
use ff::PrimeField;
use poseidon_rs::Fr;

use r2d2_redis::redis::Commands;
use serde::Deserialize;
use serde_json::json;
use std::process::Command;
use warp::reject::Reject;
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
    pool: Pool<RedisConnectionManager>,
) -> Result<Json, warp::Rejection> {
    let election = Election::new(data.sign_up_duration, data.voting_duration);

    Ok(warp::reply::json(&election))
}

pub async fn sign_up(
    data: ElectionSignUp,
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
    //TO DO: Get the votes from the blockchain
    let votes = [
        1, 2, 3, 2, 2, 3, 1, 2, 1, 2, 3, 2, 2, 3, 1, 2, 1, 2, 3, 2, 2, 3, 1, 2, 1, 2, 3, 2, 2, 3,
        1, 2,
    ];

    const MAX_AMOUNT_OF_VOTES: isize = 32;

    let mut con = match pool.get() {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    let votes_from_pool: Vec<String> = match con.lrange("votes", 0, MAX_AMOUNT_OF_VOTES - 1) {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    println!("Votes from pool : {:?}", votes_from_pool);

    //TO DO: Calculate the merkle root with the votes
    let votes_merkle_root_fr_str = generate_merkle_root(
        votes
            .map(|v| Fr::from_str(&v.to_string()).unwrap())
            .to_vec(),
    )
    .to_string();

    let votes_merkle_root_leo_str = leo_io::fr_string_to_leo_str(votes_merkle_root_fr_str);

    leo_io::generate_input_file(votes, votes_merkle_root_leo_str.as_str());

    //TO DO: Make async, don't run if it's already running or has run before
    Command::new("sh")
        .arg("-c")
        .arg("make run_circuits")
        .output()
        .expect("failed to execute process");

    Ok(warp::reply::json(&json!({"msg":"Tally has begun"})))
}
