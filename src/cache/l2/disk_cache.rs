use rocksdb::{DB, Options, WriteBatch};
use std::path::Path;
use std::sync::Arc;
use bytes::Bytes;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct CacheEntry {
    data: Vec<u8>,
    created_at: u64,   // Unix timestamp (seconds)
    ttl_seconds: u64,
}

impl CacheEntry {
    fn is_expired(&self, now: u64) -> bool {
        now > self.created_at + self.ttl_seconds
    }
}

pub struct DiskCache {
    db: Arc<DB>,
    ttl_seconds: u64,
}

impl DiskCache {
    pub fn new(path: &str, ttl_seconds: u64) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
        let db = DB::open(&opts, path).expect("Failed to open RocksDB");
        Self {
            db: Arc::new(db),
            ttl_seconds,
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        match self.db.get(key.as_bytes()) {
            Ok(Some(value)) => {
                match bincode::deserialize::<CacheEntry>(&value) {
                    Ok(entry) if !entry.is_expired(now) => Some(entry.data),
                    Ok(_) => {
                        // Expired, delete
                        let _ = self.db.delete(key.as_bytes());
                        None
                    }
                    Err(_) => None,
                }
            }
            _ => None,
        }
    }

    pub async fn set(&self, key: &str, value: Vec<u8>) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let entry = CacheEntry {
            data: value,
            created_at: now,
            ttl_seconds: self.ttl_seconds,
        };
        if let Ok(serialized) = bincode::serialize(&entry) {
            let _ = self.db.put(key.as_bytes(), serialized);
        }
    }

    pub async fn remove(&self, key: &str) {
        let _ = self.db.delete(key.as_bytes());
    }

    pub async fn flush_expired(&self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut iter = self.db.iterator(rocksdb::IteratorMode::Start);
        let mut batch = WriteBatch::default();
        while let Some(Ok((key, value))) = iter.next() {
            if let Ok(entry) = bincode::deserialize::<CacheEntry>(&value) {
                if entry.is_expired(now) {
                    batch.delete(key);
                }
            }
        }
        let _ = self.db.write(batch);
    }
}

impl Clone for DiskCache {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            ttl_seconds: self.ttl_seconds,
        }
    }
}