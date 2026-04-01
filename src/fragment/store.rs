use std::sync::Arc;
use dashmap::DashMap;
use super::types::Fragment;

pub struct FragmentStore {
    fragments: Arc<DashMap<String, Fragment>>, // key -> fragment
}

impl FragmentStore {
    pub fn new() -> Self {
        Self {
            fragments: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Fragment> {
        self.fragments.get(key).map(|f| f.clone())
    }

    pub fn set(&self, key: String, fragment: Fragment) {
        self.fragments.insert(key, fragment);
    }

    pub fn remove(&self, key: &str) {
        self.fragments.remove(key);
    }

    pub fn contains(&self, key: &str) -> bool {
        self.fragments.contains_key(key)
    }
}

impl Default for FragmentStore {
    fn default() -> Self {
        Self::new()
    }
}