use chrono::{DateTime, Utc};

mod memory;

pub use memory::MemoryStore;

#[derive(Debug, Clone)]
pub struct KvElement {
    pub data: Vec<u8>,
    pub mime_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
    pub update_count: i32,
    pub locked: bool,
}