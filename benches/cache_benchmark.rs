use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lru_cache::Cache;
use lru_cache::cache::traits::CacheStorage;

fn cache_operations(c: &mut Criterion) {
    c.bench_function("put_operation", |b| {
        let mut cache = Cache::new(1000);
        let mut i = 0;
        b.iter(|| {
            cache.put(black_box(i), black_box(format!("value_{}", i)));
            i = (i + 1) % 2000;
        })
    });

    c.bench_function("get_operation", |b| {
        let mut cache = Cache::new(1000);
        for i in 0..1000 {
            cache.put(i, format!("value_{}", i));
        }
        let mut i = 0;
        b.iter(|| {
            black_box(cache.get(&black_box(i)));
            i = (i + 1) % 1500;
        })
    });

    c.bench_function("mixed_operations", |b| {
        let mut cache = Cache::new(1000);
        let mut i = 0;
        b.iter(|| {
            if i % 2 == 0 {
                cache.put(black_box(i), black_box(format!("value_{}", i)));
            } else {
                black_box(cache.get(&black_box(i - 1)));
            }
            i = (i + 1) % 2000;
        })
    });
}

criterion_group!(benches, cache_operations);
criterion_main!(benches);