use std::collections::HashMap;
use std::sync::Arc;
use dashmap::DashMap;
use super::placeholder::Placeholder;

pub struct TemplateCache {
    templates: Arc<DashMap<String, String>>, // key -> template with placeholders
}

impl TemplateCache {
    pub fn new() -> Self {
        Self {
            templates: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.templates.get(key).map(|v| v.clone())
    }

    pub fn set(&self, key: String, template: String) {
        self.templates.insert(key, template);
    }

    pub fn remove(&self, key: &str) {
        self.templates.remove(key);
    }
}

impl Default for TemplateCache {
    fn default() -> Self {
        Self::new()
    }
}