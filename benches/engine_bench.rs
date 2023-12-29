use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use tiny_kv::engine::sled::SledEngine;
use tiny_kv::engine::mem::MemEngine;
use sled;
use tempfile::TempDir;
use tiny_kv::engine::KvEngine;
use rand::prelude::*;

const SEED: [u8; 32] = [1,0,0,0, 23,0,0,0, 200,1,0,0, 210,30,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];

fn set_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_bench");
    group.sample_size(10);
    group.bench_function("sled", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                (SledEngine::new(sled::open(&temp_dir).unwrap()), temp_dir)
            },
            |(engine, _temp_dir)| {
                for i in 1..(1 << 12) {
                    engine.set(format!("key{}", i), "value".to_string()).unwrap();
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("mem", |b| {
        b.iter_batched(
            || MemEngine::new(),
            |engine| {
                for i in 1..(1 << 12) {
                    engine.set(format!("key{}", i), "value".to_string()).unwrap();
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn get_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_bench");
    group.sample_size(10);
    for i in &vec![8, 12, 16, 20] {
        group.bench_with_input(format!("mem_{}", i), i, |b, i| {
            let engine = MemEngine::new();

            for key_i in 1..(1 << i) {
                engine.set(format!("key{}", key_i), "value".to_string()).unwrap();
            }

            let mut rng =  StdRng::from_seed(SEED);
            b.iter(|| {
                engine.get(format!("key{}", rng.gen_range(1..(1 << i))))
            });
        });
    }

    for i in &vec![8, 12, 16, 20] {
        group.bench_with_input(format!("sled_{}", i), i, |b, i| {
            let temp_dir = TempDir::new().unwrap();
            let engine = SledEngine::new(sled::open(&temp_dir).unwrap());

            for key_i in 1..(1 << i) {
                engine.set(format!("key{}", key_i), "value".to_string()).unwrap();
            }

            let mut rng =  StdRng::from_seed(SEED);
            b.iter(|| {
                engine.get(format!("key{}", rng.gen_range(1..(1 << i))))
            });
        });
    }
    group.finish();
}

criterion_group!(benches, set_bench, get_bench);
criterion_main!(benches);
