use std::time::{Instant};
use tempfile::TempDir;
use tiny_kv::engine::KvEngine;
use tiny_kv::engine::mem::MemEngine;
use tiny_kv::engine::sled::SledEngine;

#[test]
fn test_mem_engine() {
    let engine = MemEngine::new();
    test_kv(engine);
}

#[test]
fn test_sled_engine() {
    let temp_dir = TempDir::new().unwrap();
    let engine = SledEngine::new(sled::open(&temp_dir).unwrap());
    test_kv(engine);
}

fn test_kv<T: KvEngine>(engine: T) {
    let key = String::from("name");
    let value = String::from("James");
    // test get None value
    assert_eq!(engine.get(key.clone()).unwrap(), None);

    // test set value
    engine.set(key.clone(), value.clone()).unwrap();
    assert_eq!(engine.get(key.clone()).unwrap(), Some(value.clone()));

    // test remove key
    engine.remove(key.clone()).unwrap();
    assert_eq!(engine.get(key.clone()).unwrap(), None);
}