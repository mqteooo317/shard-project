use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use dashmap::DashMap;
use super::key::CacheKey;
use super::eviction::{EntryMetadata, EvictionPolicy};

pub struct ShardCache {
    shards: Vec<Arc<DashMap<CacheKey, (Vec<u8>, EntryMetadata)>>>,
    policy: EvictionPolicy,
}

impl ShardCache {
    pub fn new(num_shards: usize, max_memory_bytes: usize, ttl_seconds: u64) -> Self {
        let mut shards = Vec::with_capacity(num_shards);
        for _ in 0..num_shards {
            shards.push(Arc::new(DashMap::new()));
        }
        Self {
            shards,
            policy: EvictionPolicy::new(max_memory_bytes, ttl_seconds),
        }
    }

    fn shard_index(&self, key: &CacheKey) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.shards.len()
    }

    pub fn get(&self, key: &CacheKey) -> Option<Vec<u8>> {
        let shard = &self.shards[self.shard_index(key)];
        if let Some(entry) = shard.get(key) {
            let (data, metadata) = entry.value();
            if self.policy.is_expired(metadata) {
                drop(entry);
                shard.remove(key);
                None
            } else {
                metadata.record_access();
                Some(data.clone())
            }
        } else {
            None
        }
    }

    pub fn set(&self, key: CacheKey, value: Vec<u8>) {
        let shard = &self.shards[self.shard_index(&key)];
        let metadata = EntryMetadata::new();
        shard.insert(key, (value, metadata));
        // TODO: trigger eviction if memory limit exceeded
    }

    pub fn remove(&self, key: &CacheKey) {
        let shard = &self.shards[self.shard_index(key)];
        shard.remove(key);
    }

    pub fn evict_expired(&self) {
        for shard in &self.shards {
            let keys_to_remove: Vec<CacheKey> = shard
                .iter()
                .filter_map(|entry| {
                    let (_, metadata) = entry.value();
                    if self.policy.is_expired(metadata) {
                        Some(entry.key().clone())
                    } else {
                        None
                    }
                })
                .collect();
            for key in keys_to_remove {
                shard.remove(&key);
            }
        }
    }
}

impl Clone for ShardCache {
    fn clone(&self) -> Self {
        Self {
            shards: self.shards.clone(),
            policy: self.policy.clone(),
        }
    }
}