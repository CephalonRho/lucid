use lucid::kvstore::{Encryption, MemoryStore, Store};

const CIPHER: &str = "123456789012345678901234123456789012345678901234";

const DATA: [u8; 512] = [42u8; 512];

const KEY: &str = "test_value";

async fn init_kv() -> MemoryStore {
    let kv = MemoryStore::new(Some(Encryption::serpent(hex::decode(CIPHER).unwrap())));
    kv.set(KEY.to_string(), DATA.to_vec()).await;
    kv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_returns_a_value() {
        let kv = init_kv().await;
        let value = kv.get(KEY.to_string()).await;

        match value {
            Some(v) => assert_eq!(v.data, DATA.to_vec()),
            None => panic!("No value found"),
        }
    }
}
