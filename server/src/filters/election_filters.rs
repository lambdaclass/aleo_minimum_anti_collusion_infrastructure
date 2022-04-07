use crate::controllers::election_controllers;
use crate::services::kvstore::with_db;
use crate::RocksDB;
use warp::Filter;

pub fn get_filters(
    db: RocksDB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let election_base = warp::path("election");

    let create = election_base
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(election_controllers::create);

    let get = election_base
        .and(warp::get())
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(election_controllers::get);

    create.or(get)
}
