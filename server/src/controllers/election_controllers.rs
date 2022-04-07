use crate::models::Election;
use crate::services::kvstore::{KVStore, RocksDB};
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

pub async fn create(data: ElectionCreate, db: RocksDB) -> Result<Json, warp::Rejection> {
    let election = Election::new(data.sign_up_duration, data.voting_duration);
    //call ledger here
    let aleo_transaction_id: &str = "ABC123";
    let saved = db.save(election.get_id(), aleo_transaction_id);
    if saved {
        Ok(warp::reply::json(&election))
    } else {
        Err(warp::reject())
    }
}

pub async fn get(data: ElectionSignUp) -> Result<Json, warp::Rejection> {
    Ok(warp::reply::json(&json!({"msg":"not implemented"})))
}
