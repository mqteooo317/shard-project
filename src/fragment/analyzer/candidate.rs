use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub id: String,
    pub selector: String,
    pub content: String,
    pub dependencies: HashSet<String>,
    pub hash: u64,
}

impl Candidate {
    pub fn new(id: String, selector: String, content: String) -> Self {
        let hash = Self::compute_hash(&content);
        Self {
            id,
            selector,
            content,
            dependencies: HashSet::new(),
            hash,
        }
    }

    fn compute_hash(content: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&content, &mut hasher);
        std::hash::Hasher::finish(&hasher)
    }
}