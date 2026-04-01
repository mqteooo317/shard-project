use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Fragment {
    pub id: String,
    pub content: String,
    pub hash: u64,
    pub dependencies: HashSet<String>,
}

impl Fragment {
    pub fn new(id: String, content: String) -> Self {
        let hash = Self::compute_hash(&content);
        Self {
            id,
            content,
            hash,
            dependencies: HashSet::new(),
        }
    }

    fn compute_hash(content: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&content, &mut hasher);
        std::hash::Hasher::finish(&hasher)
    }

    pub fn add_dependency(&mut self, dep: String) {
        self.dependencies.insert(dep);
    }
}