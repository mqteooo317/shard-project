#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Placeholder {
    pub id: String,
    pub marker: String,
}

impl Placeholder {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            marker: format!("__SHARD_FRAGMENT_{}__", id),
        }
    }

    pub fn to_marker(&self) -> String {
        self.marker.clone()
    }
}