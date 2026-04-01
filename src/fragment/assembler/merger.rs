use std::collections::HashMap;
use super::placeholder::Placeholder;

pub struct Merger;

impl Merger {
    pub fn merge(template: &str, fragments: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        for (id, content) in fragments {
            let placeholder = Placeholder::new(id);
            result = result.replace(&placeholder.to_marker(), content);
        }
        result
    }
}