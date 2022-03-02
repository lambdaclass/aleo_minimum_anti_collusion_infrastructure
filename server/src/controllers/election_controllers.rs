use crate::models::Election;
use serde::Deserialize;
use warp::reply::Json;

#[derive(Debug, Deserialize)]
pub struct ElectionCreate {
    sign_up_duration: i64,
    voting_duration: i64,
}

pub async fn create(data: ElectionCreate) -> Result<Json, warp::Rejection> {
    let election = Election::new(data.sign_up_duration, data.voting_duration);

    Ok(warp::reply::json(&election))
}
