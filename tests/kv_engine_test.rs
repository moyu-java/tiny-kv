use std::env;
use tiny_kv::engine::KvEngine;
use tiny_kv::engine::mem::MemEngine;
use tiny_kv::engine::sled::SledEngine;

#[test]
fn test_mem_engine() {
    let mem_engine = MemEngine::new();
    test_kv(mem_engine);
}

#[test]
fn test_sled_engine() {
    let sled_engine = SledEngine::new(sled::open(env::current_dir().unwrap()).unwrap());
    test_kv(sled_engine);
}

fn test_kv<T: KvEngine>(engine: T) {
    let key = String::from("name");
    let value = String::from("James");
    engine.set(key.clone(), value.clone()).unwrap();
    assert_eq!(engine.get(key.clone()).unwrap(), Some(value.clone()));
    engine.remove(key.clone()).unwrap();
    assert_eq!(engine.get(key.clone()).unwrap(), None);
}