mod controllers;
mod filters;
mod models;
mod services;

use crate::filters::election_filters;

#[tokio::main] // 2.
async fn main() {
    let filters = election_filters::get_filters();

    warp::serve(filters) // 5.
        .run(([127, 0, 0, 1], 3000)) // 6.
        .await;
}
