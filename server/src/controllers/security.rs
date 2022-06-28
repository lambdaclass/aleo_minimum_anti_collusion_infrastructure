use std::env;
use warp::Filter;

const HEADER_XAUTH: &str = "X-Auth-Token";

pub fn check_auth() -> impl Filter<Extract = ((),), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::<String>(HEADER_XAUTH))
        .and_then(|xauth: String| async move {
            let token = env::var("ADMIN_TOKEN").expect("$ADMIN_TOKEN not setted");
            if xauth != token {
                return Err(warp::reject::custom(FailAuth));
            }

            Ok::<(), warp::Rejection>(())
        })
}
#[derive(Debug)]
pub struct FailAuth;

impl warp::reject::Reject for FailAuth {}
