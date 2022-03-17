mod controllers;
mod filters;
mod models;
mod services;

use crate::filters::election_filters;
use crate::services::kvstore::{KVStore, RocksDB};

#[tokio::main] // 2.
async fn main() {
    let filters = election_filters::get_filters();
    let db: RocksDB = KVStore::init("./.tmp/rocksdb");

    warp::serve(filters) // 5.
        .run(([127, 0, 0, 1], 3000)) // 6.
        .await;
}
