use crate::r2d2::Pool;
use crate::RedisConnectionManager;
use crate::{models::Election, services::leo_io};
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
    let mut con = pool.get().unwrap();
    let _: () = con.lpush("votes", &data.aleo_transaction_id).unwrap();

    Ok(warp::reply::json(
        &json!({"msg":"your vote was succesuffly stored"}),
    ))
}

pub async fn start_tally() -> Result<Json, warp::Rejection> {
    //TO DO: Get the votes from the blockchain
    let votes = [
        1, 2, 3, 2, 2, 3, 1, 2, 1, 2, 3, 2, 2, 3, 1, 2, 1, 2, 3, 2, 2, 3, 1, 2, 1, 2, 3, 2, 2, 3,
        1, 2,
    ];

    //TO DO: Calculate the merkle root with the votes
    let votes_merke_root =
        "6081127065217055003429398673533374549058098389318475736416753929574343365699";

    leo_io::generate_input_file(votes, votes_merke_root);

    //TO DO: Make async, don't run if it's already running or has run before
    Command::new("sh")
        .arg("-c")
        .arg("make run_circuits")
        .output()
        .expect("failed to execute process");

    Ok(warp::reply::json(&json!({"msg":"Tally has begun"})))
}
