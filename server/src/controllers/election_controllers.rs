use crate::models::Election;
use crate::r2d2::Pool;
use crate::RedisConnectionManager;
use r2d2_redis::redis::Commands;
use serde::Deserialize;
use serde_json::json;
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
    public_key_old: String,
    public_key_new: String,
    vote_option: String,
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
    let _: () = con.lpush("votes", "random_id").unwrap();

    Ok(warp::reply::json(&json!({"msg":"vote stored"})))
}
