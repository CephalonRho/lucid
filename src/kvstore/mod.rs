use async_trait::async_trait;
use block_modes::{block_padding::ZeroPadding, BlockMode, Cbc};
use chrono::{DateTime, Utc};
use serpent::Serpent;

mod memory;

pub use memory::MemoryStore;

#[async_trait]
pub trait Store {
    async fn get(&self, key: String) -> Option<KvElement>;
    async fn set(&self, key: String, mut value: Vec<u8>) -> Option<KvElement>;
    async fn delete(&self, key: String);
    async fn set_lock(&self, key: String, lock: bool) -> bool;
    async fn add(&self, key: String, addend: f64) -> bool;
}

#[derive(Debug, Clone)]
pub struct KvElement {
    pub data: Vec<u8>,
    pub mime_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
    pub update_count: i32,
    pub locked: bool,
    pub iv: Option<Vec<u8>>,
}

pub enum Encryption {
    Serpent { key: Vec<u8> },
}

impl Encryption {
    pub fn serpent(key: Vec<u8>) -> Self {
        Self::Serpent { key }
    }

    #[allow(dead_code)]
    pub fn encrypt<'a>(
        &self,
        buffer: &'a mut [u8],
        pos: usize,
        iv: &[u8],
    ) -> Result<&'a [u8], Box<dyn std::error::Error>> {
        match self {
            Self::Serpent { key } => {
                let serpent =
                    Cbc::<Serpent, ZeroPadding>::new_var(&key, &iv).map_err(|e| Box::new(e))?;
                serpent
                    .encrypt(buffer, pos)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }

    pub fn encrypt_vec(
        &self,
        plaintext: &[u8],
        iv: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match self {
            Self::Serpent { key } => {
                let serpent =
                    Cbc::<Serpent, ZeroPadding>::new_var(&key, &iv).map_err(|e| Box::new(e))?;
                Ok(serpent.encrypt_vec(plaintext))
            }
        }
    }

    #[allow(dead_code)]
    pub fn decrypt<'a>(
        &self,
        buffer: &'a mut [u8],
        iv: &[u8],
    ) -> Result<&'a [u8], Box<dyn std::error::Error>> {
        match self {
            Self::Serpent { key } => {
                let serpent =
                    Cbc::<Serpent, ZeroPadding>::new_var(&key, &iv).map_err(|e| Box::new(e))?;
                serpent
                    .decrypt(buffer)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }

    pub fn decrypt_vec(
        &self,
        ciphertext: &[u8],
        iv: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match self {
            Self::Serpent { key } => {
                let serpent =
                    Cbc::<Serpent, ZeroPadding>::new_var(&key, &iv).map_err(|e| Box::new(e))?;
                serpent
                    .decrypt_vec(ciphertext)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }
}
