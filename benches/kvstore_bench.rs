use criterion::{criterion_group, criterion_main, Criterion};

use lucid::kvstore::{Encryption, MemoryStore};

const CIPHER: &str = "123456789012345678901234123456789012345678901234";

const DATA: [u8; 1000] = [42u8; 1000];

fn set_1_kb_data(c: &mut Criterion) {
    let kv = MemoryStore::new(Some(Encryption::serpent(hex::decode(CIPHER).unwrap())));

    c.bench_function("Set 1KB", |b| {
        b.iter(|| kv.set("bench_one".to_string(), DATA.to_vec()))
    });
}
fn get_1_kb_data(c: &mut Criterion) {
    let kv = MemoryStore::new(Some(Encryption::serpent(hex::decode(CIPHER).unwrap())));

    let k = String::from("bench_one");
    kv.set(k.clone(), DATA.to_vec());

    c.bench_function("Get 1KB", |b| b.iter(|| kv.get(k.clone())));
}

fn set_1_kb_data_without_encryption(c: &mut Criterion) {
    let kv = MemoryStore::new(None);

    c.bench_function("Set 1KB (w/o encrytion)", |b| {
        b.iter(|| kv.set("bench_one".to_string(), DATA.to_vec()))
    });
}
fn get_1_kb_data_without_encryption(c: &mut Criterion) {
    let kv = MemoryStore::new(None);

    let k = String::from("bench_one");
    kv.set(k.clone(), DATA.to_vec());

    c.bench_function("Get 1KB (w/o encryption)", |b| b.iter(|| kv.get(k.clone())));
}

criterion_group!(
    benches,
    set_1_kb_data,
    get_1_kb_data,
    set_1_kb_data_without_encryption,
    get_1_kb_data_without_encryption
);
criterion_main!(benches);
