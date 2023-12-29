use sled::{Db, Tree};
use crate::engine::KvEngine;
use crate::KvError;

#[derive(Clone)]
pub struct SledEngine(Db);

impl SledEngine {
    pub fn new(db: Db) -> Self {
        SledEngine(db)
    }
}

impl KvEngine for SledEngine {
    fn set(&self, key: String, value: String) -> crate::Result<()> {
        let tree: &Tree = &self.0;
        tree.insert(key, value.into_bytes()).map(|_| ())?;
        tree.flush()?;
        Ok(())
    }

    fn get(&self, key: String) -> crate::Result<Option<String>> {
        let tree: &Tree = &self.0;
        Ok(tree
            .get(key)?
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&self, key: String) -> crate::Result<()> {
        let tree: &Tree = &self.0;
        tree.remove(key)?.ok_or(KvError::KeyNotFound)?;
        tree.flush()?;
        Ok(())
    }
}