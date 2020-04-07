use chashmap::CHashMap;
use chrono::Utc;

use super::{Encryption, KvElement};

pub struct MemoryStore {
    container: CHashMap<String, KvElement>,
    encryption: Option<Encryption>,
}

impl MemoryStore {
    pub fn new(encryption: Option<Encryption>) -> Self {
        Self {
            container: CHashMap::new(),
            encryption,
        }
    }

    pub fn set(&self, key: String, mut value: Vec<u8>) -> Option<KvElement> {
        let iv = if let Some(encryption) = &self.encryption {
            let iv = rand::random::<[u8; 16]>();
            value = encryption.encrypt_vec(&value, &iv).unwrap();
            Some(iv)
        } else {
            None
        };
        match &mut self.container.get_mut(&key) {
            Some(kv_element) => {
                if !kv_element.locked {
                    let mime_type = tree_magic::from_u8(value.as_ref());
                    kv_element.data = value;
                    kv_element.mime_type = mime_type;
                }
                kv_element.updated_at = Utc::now();
                kv_element.update_count = kv_element.update_count + 1;
                Some(kv_element.to_owned())
            }
            None => {
                let mime_type = tree_magic::from_u8(value.as_ref());
                let kv_element = KvElement {
                    data: value,
                    mime_type,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    expire_at: Utc::now(),
                    update_count: 1,
                    locked: false,
                    iv: iv.map(|x| x.to_vec()),
                };
                self.container.insert(key, kv_element)
            }
        }
    }

    pub fn get(&self, key: String) -> Option<KvElement> {
        match self.container.get(&key) {
            Some(value) => {
                let mut cloned_value = value.clone();

                if let Some(encryption) = &self.encryption {
                    let iv = value
                        .iv
                        .as_ref()
                        .expect("encryption is enabled but element doesn't have IV");
                    cloned_value.data = encryption.decrypt_vec(&value.data, iv).unwrap();
                }
                Some(cloned_value)
            }
            None => None,
        }
    }

    pub fn switch_lock(&self, key: String, to_lock: bool) -> bool {
        match &mut self.container.get_mut(&key) {
            Some(kv_element) => {
                kv_element.locked = to_lock;
                true
            }
            None => false,
        }
    }

    pub fn increment_or_decrement(&self, key: String, value: f64) -> bool {
        match &mut self.container.get_mut(&key) {
            Some(kv_element) => {
                let byte_to_string = String::from_utf8(kv_element.clone().data).unwrap(); // TODO: handle convert to string error
                match byte_to_string.trim().parse::<f64>() {
                    Ok(initial_value) => {
                        kv_element.data = (initial_value + value).to_string().into_bytes();
                        kv_element.updated_at = Utc::now();
                        kv_element.update_count = kv_element.update_count + 1;
                        true
                    }
                    Err(_) => false,
                }
            }
            None => false,
        }
    }

    // TODO: implement Lock, Unlock, Increment, Decrement, Expire

    pub fn drop(&self, key: String) {
        self.container.remove(&key);
    }
}
