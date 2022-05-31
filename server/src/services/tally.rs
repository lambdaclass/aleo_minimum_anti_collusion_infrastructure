use aleo_maci_libs::rcp::{get_transaction_public_data, public_data_to_vote};
use r2d2_redis::{r2d2::Pool, redis::Commands, RedisConnectionManager};

use crate::{models::Tally, utils::errors::TallyError};

pub async fn calculate(pool: Pool<RedisConnectionManager>) -> Result<Tally, TallyError> {
    // get redis pool connection
    let mut con = match pool.get() {
        Ok(v) => v,
        Err(_) => return Err(TallyError),
    };

    // get transaction id from redis db
    let votes_ids_from_pool: Vec<String> = match con.lrange("votes", 0, -1) {
        Ok(v) => v,
        Err(_) => return Err(TallyError),
    };

    // Get votes from the Aleo Ledger
    let mut votes: Vec<String> = Vec::new();
    for v in votes_ids_from_pool {
        let public_data = get_transaction_public_data(v.to_string()).await;
        match public_data {
            Ok(data) => votes.push(public_data_to_vote(data)),
            Err(_) => return Err(TallyError),
        };
    }

    let tally = Tally::new(
        votes
            .clone()
            .into_iter()
            .map(|v| v.parse::<u32>().unwrap())
            .collect(),
    );

    //strore result on cache
    let _: () = con.del("results").unwrap();
    let _: () = con.rpush("results", tally.results.clone()).unwrap();

    Ok(tally)
}
