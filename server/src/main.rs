use warp::Filter;

mod models;

use self::models::Election;

#[tokio::main] // 2.
async fn main() {
    let hello = warp::path!("election" / i64 / i64) // 3.
        .map(|sign_up_duration, voting_duration| {
            warp::reply::json(&Election::new(sign_up_duration, voting_duration))
        }); // 4.

    warp::serve(hello) // 5.
        .run(([127, 0, 0, 1], 3000)) // 6.
        .await;
}
