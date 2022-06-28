mod controllers;
mod models;
mod services;
mod utils;

use crate::controllers::{election_controllers, security::check_auth};
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::env;
use warp::Filter;

const APP_PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    let host = env::var("HOST").expect("$HOST not setted");
    let redis_url = env::var("REDIS_URL").expect("$REDIS_URL not setted");

    //to test if the env variable has been set
    env::var("ADMIN_TOKEN").expect("$ADMIN_TOKEN not setted");

    let host_for_warp = match host.as_str() {
        "docker" => [0, 0, 0, 0],
        _ => [127, 0, 0, 1],
    };

    println!("HOST {}:{}", host, APP_PORT);
    println!("REDIS_URL {}", redis_url);

    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let warp_pool = warp::any().map(move || pool.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT"]);

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

    let create_msg = election_base
        .and(warp::post())
        .and(warp::path("msg"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp_pool.clone())
        .and_then(election_controllers::store_msg);

    let get_votes = election_base
        .and(warp::get())
        .and(warp::path("votes"))
        .and(warp::path::end())
        .and(warp_pool.clone())
        .and_then(election_controllers::get_votes);

    let start_tally = election_base
        .and(warp::post())
        .and(warp::path("tally"))
        .and(warp::path("start"))
        .and(warp::path::end())
        .and(warp_pool.clone())
        .and_then(election_controllers::start_tally);

    let create_whitelist = election_base
        .and(warp::post())
        .and(warp::path("whitelist"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(check_auth().untuple_one())
        .and(warp_pool.clone())
        .and_then(election_controllers::create_whitelist);

    let get_whitelist = election_base
        .and(warp::get())
        .and(warp::path("whitelist"))
        .and(warp::path::end())
        .and(warp_pool.clone())
        .and_then(election_controllers::get_whitelist);

    let get_results = election_base
        .and(warp::get())
        .and(warp::path("tally"))
        .and(warp::path("results"))
        .and(warp::path::end())
        .and(warp_pool.clone())
        .and_then(election_controllers::get_results);

    let filters = create
        .or(sign_up)
        .or(create_msg)
        .or(get_votes)
        .or(start_tally)
        .or(create_whitelist)
        .or(get_whitelist)
        .or(get_results)
        .with(cors);

    warp::serve(filters) // 5.
        .run((host_for_warp, APP_PORT)) // 6.
        .await;
}
