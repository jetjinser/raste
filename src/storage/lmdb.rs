use heed::{types::Str, Database, Env, EnvOpenOptions};
use std::{fs, path::Path as fsPath};

use crate::keygen::gen_key;

use super::Storage;

#[derive(Clone)]
pub struct DBEngine {
    env: Env,
    db: Database<Str, Str>,
}

impl Storage for DBEngine {
    fn new(path: &fsPath) -> Self {
        fs::create_dir_all(path).unwrap();
        let env = EnvOpenOptions::new().max_dbs(1).open(path).unwrap();

        let db: Database<Str, Str> = env
            .open_database(Some("docs"))
            .unwrap()
            .unwrap_or_else(|| env.create_database(Some("docs")).unwrap());

        Self { env, db }
    }

    fn write_doc(&self, doc: &str) -> String {
        let mut key = gen_key();

        let rx = self.env.typed_read_txn::<Str>().unwrap();
        while self.db.get(&rx, &key).unwrap().is_some() {
            key = gen_key();
        }
        rx.commit().unwrap();

        let mut wx = self.env.typed_write_txn::<Str>().unwrap();
        self.db.put(&mut wx, &key, doc).unwrap();
        wx.commit().unwrap();

        key
    }

    fn read_doc(&self, key: &str) -> Option<String> {
        let rx = self.env.typed_read_txn::<Str>().unwrap();
        self.db.get(&rx, key).unwrap().map(|x| x.to_string())
    }
}
