mod controllers;
mod models;
mod services;

use crate::controllers::election_controllers;
use r2d2_redis::{r2d2, RedisConnectionManager};
use warp::Filter;

#[tokio::main] // 2.
async fn main() {
    let manager = RedisConnectionManager::new("redis://127.0.0.1:6379").unwrap();
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

    let filters = create.or(sign_up).or(msg);

    warp::serve(filters) // 5.
        .run(([127, 0, 0, 1], 3000)) // 6.
        .await;
}
