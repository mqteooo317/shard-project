use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheKey {
    pub url: String,
    pub fragment_id: Option<String>,
    pub variant: Option<String>,
}

impl CacheKey {
    pub fn new(url: String) -> Self {
        Self {
            url,
            fragment_id: None,
            variant: None,
        }
    }

    pub fn fragment(url: String, fragment_id: String) -> Self {
        Self {
            url,
            fragment_id: Some(fragment_id),
            variant: None,
        }
    }

    pub fn with_variant(mut self, variant: String) -> Self {
        self.variant = Some(variant);
        self
    }
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        self.fragment_id.hash(state);
        self.variant.hash(state);
    }
}