use std::path::Path as fsPath;

pub trait Storage {
    fn new(path: &fsPath) -> Self;

    fn write_doc(&self, doc: &str) -> String;
    fn read_doc(&self, key: &str) -> Option<String>;
}

#[cfg(feature = "rocksdb")]
#[path = ""]
mod export {
    mod rocksdb;
    pub use rocksdb::DBEngine;
}

#[cfg(feature = "lmdb")]
#[path = ""]
mod export {
    mod lmdb;
    pub use lmdb::DBEngine;
}

pub use export::DBEngine;
