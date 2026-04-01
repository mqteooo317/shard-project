use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use dashmap::DashMap;

pub struct DependencyGraph {
    // page_url -> set of fragment ids
    page_fragments: Arc<DashMap<String, HashSet<String>>>,
    // fragment_id -> set of page urls that depend on it
    fragment_pages: Arc<DashMap<String, HashSet<String>>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            page_fragments: Arc::new(DashMap::new()),
            fragment_pages: Arc::new(DashMap::new()),
        }
    }

    pub fn add_dependency(&self, page_url: String, fragment_id: String) {
        self.page_fragments
            .entry(page_url.clone())
            .or_insert_with(HashSet::new)
            .insert(fragment_id.clone());

        self.fragment_pages
            .entry(fragment_id)
            .or_insert_with(HashSet::new)
            .insert(page_url);
    }

    pub fn get_dependents(&self, fragment_id: &str) -> HashSet<String> {
        self.fragment_pages
            .get(fragment_id)
            .map(|set| set.clone())
            .unwrap_or_default()
    }

    pub fn get_fragments_for_page(&self, page_url: &str) -> HashSet<String> {
        self.page_fragments
            .get(page_url)
            .map(|set| set.clone())
            .unwrap_or_default()
    }

    pub fn remove_page(&self, page_url: &str) {
        if let Some(fragments) = self.page_fragments.remove(page_url) {
            for frag_id in fragments.1 {
                if let Some(mut pages) = self.fragment_pages.get_mut(&frag_id) {
                    pages.remove(page_url);
                    if pages.is_empty() {
                        self.fragment_pages.remove(&frag_id);
                    }
                }
            }
        }
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}