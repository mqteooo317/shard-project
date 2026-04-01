use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct EntryMetadata {
    pub last_access: SystemTime,
    pub access_count: AtomicU64,
    pub created_at: SystemTime,
}

impl EntryMetadata {
    pub fn new() -> Self {
        let now = SystemTime::now();
        Self {
            last_access: now,
            access_count: AtomicU64::new(1),
            created_at: now,
        }
    }

    pub fn record_access(&self) {
        self.access_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn access_count(&self) -> u64 {
        self.access_count.load(Ordering::Relaxed)
    }
}

impl Default for EntryMetadata {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EvictionPolicy {
    pub max_memory_bytes: usize,
    pub ttl: Duration,
}

impl EvictionPolicy {
    pub fn new(max_memory_bytes: usize, ttl_seconds: u64) -> Self {
        Self {
            max_memory_bytes,
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub fn is_expired(&self, metadata: &EntryMetadata) -> bool {
        let now = SystemTime::now();
        let age = now.duration_since(metadata.created_at).unwrap_or(Duration::ZERO);
        age > self.ttl
    }

    pub fn score(&self, metadata: &EntryMetadata) -> f64 {
        let age = SystemTime::now()
            .duration_since(metadata.last_access)
            .unwrap_or(Duration::ZERO)
            .as_secs_f64();
        let freq = metadata.access_count() as f64;
        freq / (1.0 + age)
    }
}