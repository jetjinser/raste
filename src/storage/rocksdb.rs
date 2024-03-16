use std::sync::Arc;

use rocksdb::DB;

use crate::{keygen::gen_key, storage::Storage};

#[derive(Clone)]
pub struct DBEngine {
    db: Arc<DB>,
}

impl Storage for DBEngine {
    fn new(path: &std::path::Path) -> Self {
        let db = DB::open_default(path).map(|x| Arc::new(x)).unwrap();
        Self { db }
    }

    fn write_doc(&self, doc: &str) -> String {
        let mut key = gen_key();

        while self.db.get(&key).unwrap().is_some() {
            key = gen_key();
        }

        self.db.put(&key, doc).unwrap();

        key
    }

    fn read_doc(&self, key: &str) -> Option<String> {
        self.db
            .get(key)
            .unwrap()
            .map(|x| String::from_utf8(x).unwrap())
    }
}
