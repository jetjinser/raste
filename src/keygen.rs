use rand::distributions::{Alphanumeric, DistString};

pub fn gen_key() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 10)
}

