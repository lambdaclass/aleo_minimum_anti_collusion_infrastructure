use rocksdb::DB;
use std::sync::Arc;

pub trait KVStore {
    fn init(file_path: &str) -> Self;
    fn save(&self, k: &str, v: &str) -> bool;
    fn find(&self, k: &str) -> Option<String>;
    fn delete(&self, k: &str) -> bool;
}

#[derive(Clone)]
pub struct RocksDB {
    db: Arc<DB>,
}

impl KVStore for RocksDB {
    fn init(file_path: &str) -> Self {
        RocksDB {
            db: Arc::new(DB::open_default(file_path).unwrap()),
        }
    }

    fn save(&self, k: &str, v: &str) -> bool {
        self.db.put(k.as_bytes(), v.as_bytes()).is_ok()
    }

    fn find(&self, k: &str) -> Option<String> {
        match self.db.get(k.as_bytes()) {
            Ok(Some(v)) => {
                let result = String::from_utf8(v).unwrap();
                println!("Finding '{}' returns '{}'", k, result);
                Some(result)
            }
            Ok(None) => {
                println!("Finding '{}' returns None", k);
                None
            }
            Err(e) => {
                println!("Error retrieving value for {}: {}", k, e);
                None
            }
        }
    }

    fn delete(&self, k: &str) -> bool {
        self.db.delete(k.as_bytes()).is_ok()
    }
}
