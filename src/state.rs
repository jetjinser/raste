use std::path::Path as fsPath;

use crate::storage::Storage;

pub struct RasteState<S> {
    pub storage: S,
}

impl<S> RasteState<S>
where
    S: Storage,
{
    pub fn new(path: &fsPath) -> Self {
        let storage = S::new(path);
        Self { storage }
    }
}

impl<T> Clone for RasteState<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let storage = self.storage.clone();
        Self { storage }
    }
}
