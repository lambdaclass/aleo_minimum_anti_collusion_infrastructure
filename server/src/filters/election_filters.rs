use crate::controllers::election_controllers;
use warp::Filter;

pub fn get_filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let election_base = warp::path("election");

    let create = election_base
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(election_controllers::create);

    let sign_up = election_base
        .and(warp::post())
        .and(warp::path("sign_up"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(election_controllers::sign_up);

    let msg = election_base
        .and(warp::post())
        .and(warp::path("msg"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(election_controllers::receive_msg);
    
    let start_tally = election_base
        .and(warp::post())
        .and(warp::path("start_tally"))
        .and(warp::path::end())
        .and_then(election_controllers::start_tally);

    create.or(sign_up).or(msg).or(start_tally)
}
