use super::l1::ShardCache;
use super::l2::DiskCache;
use crate::utils::time::now_secs;

pub struct MultiLevelCache {
    pub l1: ShardCache,
    pub l2: DiskCache,
}

impl MultiLevelCache {
    pub fn new(l1_max_memory: usize, l1_ttl_secs: u64, l2_path: &str, l2_ttl_secs: u64) -> Self {
        Self {
            l1: ShardCache::new(256, l1_max_memory, l1_ttl_secs),
            l2: DiskCache::new(l2_path, l2_ttl_secs),
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        if let Some(data) = self.l1.get_str(key) {
            return Some(data);
        }
        if let Some(data) = self.l2.get(key).await {
            self.l1.set_str(key, data.clone());
            return Some(data);
        }
        None
    }

    pub async fn set(&self, key: &str, value: Vec<u8>) {
        self.l1.set_str(key, value.clone());
        self.l2.set(key, value).await;
    }
}