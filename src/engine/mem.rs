use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::engine::KvEngine;

#[derive(Clone)]
pub struct MemEngine {
    map: Arc<Mutex<HashMap<String, String>>>,
}

impl MemEngine {
    pub fn new() -> Self {
        MemEngine {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl KvEngine for MemEngine {
    fn set(&self, key: String, value: String) -> crate::Result<()> {
        let mut map = self.map.lock().unwrap();
        map.insert(key, value);
        Ok(())
    }

    fn get(&self, key: String) -> crate::Result<Option<String>> {
        let map = self.map.lock().unwrap();
        Ok(map.get(&key).cloned())
    }

    fn remove(&self, key: String) -> crate::Result<()> {
        let mut map = self.map.lock().unwrap();
        map.remove(&key);
        Ok(())
    }
}