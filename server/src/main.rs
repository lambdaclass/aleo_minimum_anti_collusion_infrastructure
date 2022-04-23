mod controllers;
mod models;
mod services;

use crate::controllers::election_controllers;
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let host = env::var("HOST").expect("$HOST not setted");
    let redis_url = env::var("REDIS_URL").expect("$REDIS_URL not setted");

    let host_for_warp = match host.as_str() {
        "docker" => [0, 0, 0, 0],
        _ => [127, 0, 0, 1],
    };

    println!("HOST {}", redis_url);
    println!("REDIS_URL {}", redis_url);

    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let warp_pool = warp::any().map(move || pool.clone());

    let election_base = warp::path("election");

    let create = election_base
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp_pool.clone())
        .and_then(election_controllers::create);

    let sign_up = election_base
        .and(warp::post())
        .and(warp::path("sign_up"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp_pool.clone())
        .and_then(election_controllers::sign_up);

    let msg = election_base
        .and(warp::post())
        .and(warp::path("msg"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp_pool.clone())
        .and_then(election_controllers::store_msg);

    let start_tally = election_base
        .and(warp::post())
        .and(warp::path("start_tally"))
        .and(warp::path::end())
        .and_then(election_controllers::start_tally);

    let filters = create.or(sign_up).or(msg).or(start_tally);

    warp::serve(filters) // 5.
        .run((host_for_warp, 3000)) // 6.
        .await;
}
