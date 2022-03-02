use crate::controllers::election_controllers;
use warp::Filter;

pub fn get_filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let election_base = warp::path("election");

    let create = election_base
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::path::end())
        .and_then(election_controllers::create);

    create
}
