use crate::models::{Election, Results, Votes, Whitelist};
use crate::r2d2::Pool;
use crate::services::leo_io::generate_input_file;
use crate::services::tally;
use crate::utils::errors::{DBError, TallyError};
use crate::utils::votes_to_fix_array;
use crate::RedisConnectionManager;

use r2d2_redis::redis::{Commands, LposOptions, RedisError};
use serde::Deserialize;
use serde_json::json;
use std::process::Command;
use warp::http::HeaderValue;
use warp::hyper::StatusCode;
use warp::reply::{Json, Response};

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
) -> Result<impl warp::Reply, warp::Rejection> {
    //TO DO: Don't store repeated values
    let mut con = match pool.get() {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    //Checks if the transaction is already stored
    let key_pos: Result<u32, RedisError> =
        con.lpos("votes", &data.aleo_transaction_id, LposOptions::default());

    match key_pos {
        Ok(_) => {
            //the transaction is already stored
            Ok(warp::reply::with_status(
                "the transaction_id has been already stored",
                StatusCode::BAD_REQUEST,
            ))
        }
        Err(_) => {
            //store the transaction
            let _: () = con.lpush("votes", &data.aleo_transaction_id).unwrap();
            Ok(warp::reply::with_status(
                "the transaction_id was stored successfully",
                StatusCode::CREATED,
            ))
        }
    }
}

pub async fn get_votes(
    pool: Pool<RedisConnectionManager>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut con = match pool.get() {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    let votes_ids_from_pool: Vec<String> = match con.lrange("votes", 0, -1) {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    Ok(warp::reply::json(&Votes::new(votes_ids_from_pool)))
}

pub async fn get_results(
    pool: Pool<RedisConnectionManager>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut con = match pool.get() {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    let results_from_pool: Vec<u32> = match con.lrange("results", 0, -1) {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    Ok(warp::reply::json(&Results::new(results_from_pool)))
}

pub async fn create_whitelist(
    data: Whitelist,
    pool: Pool<RedisConnectionManager>,
) -> Result<impl warp::Reply, warp::Rejection> {
    //TO DO: Don't store repeated values
    let mut con = match pool.get() {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    let _: () = con.del("whitelist").unwrap();
    let _: () = con.rpush("whitelist", data.accounts).unwrap();

    let body = json!({"msg": "Whitelist created successfully"});
    let mut response = Response::new(body.to_string().into());
    response
        .headers_mut()
        .insert("Content-Type", HeaderValue::from_static("application/json"));

    Ok(warp::reply::with_status(response, StatusCode::CREATED))
}

pub async fn get_whitelist(
    pool: Pool<RedisConnectionManager>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut con = match pool.get() {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    let accounts_ids_from_pool: Vec<String> = match con.lrange("whitelist", 0, -1) {
        Ok(v) => v,
        Err(_) => return Err(warp::reject::custom(DBError)),
    };

    Ok(warp::reply::json(&Whitelist::new(accounts_ids_from_pool)))
}

pub async fn start_tally(pool: Pool<RedisConnectionManager>) -> Result<Json, warp::Rejection> {
    println!("Staring a new tally ...");

    //Do the tally
    let tally = match tally::calculate(pool.clone()).await {
        Ok(t) => t,
        Err(TallyError) => return Err(warp::reject::custom(TallyError)),
    };

    println!("Tally ready: {:?}", tally);

    // Generate circuit input file
    println!("Generating circuit input ...");
    generate_input_file(votes_to_fix_array(&tally.votes), &tally.votes_markle_root);

    // Run circuit
    // TO DO: Make async, don't run if it's already running or has run before
    println!("Running circuit to verify the tally. Please wait a minute...");
    Command::new("sh")
        .arg("-c")
        .arg("cd circuits/tally;leo run")
        .output()
        .expect("failed to execute process");

    println!("Circuit execution finished.");

    Ok(warp::reply::json(&tally))
}
