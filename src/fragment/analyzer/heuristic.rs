use super::{ElementInfo, Candidate};

pub struct Heuristic;

impl Heuristic {
    pub fn is_good_candidate(element: &ElementInfo) -> bool {
        if element.id.is_some() {
            return true;
        }

        if let Some(class) = &element.class {
            if class.contains("sidebar")
                || class.contains("header")
                || class.contains("footer")
                || class.contains("menu")
                || class.contains("widget")
            {
                return true;
            }
        }

        if element.tag == "nav" || element.tag == "header" || element.tag == "footer" {
            return true;
        }

        false
    }

    pub fn should_fragment(element: &ElementInfo, _global_stats: &GlobalStats) -> bool {
        Self::is_good_candidate(element)
    }
}

pub struct GlobalStats {
    pub element_frequency: std::collections::HashMap<String, usize>,
}

impl GlobalStats {
    pub fn new() -> Self {
        Self {
            element_frequency: std::collections::HashMap::new(),
        }
    }

    pub fn record_element(&mut self, element: &ElementInfo) {
        let key = if let Some(id) = &element.id {
            format!("#{}", id)
        } else if let Some(class) = &element.class {
            format!(".{}", class)
        } else {
            element.tag.clone()
        };
        *self.element_frequency.entry(key).or_insert(0) += 1;
    }
}