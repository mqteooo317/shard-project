use std::collections::HashSet;
use super::{Candidate, ElementInfo, HtmlParser, heuristic::{Heuristic, GlobalStats}};

pub struct BoundaryDetector {
    stats: GlobalStats,
}

impl BoundaryDetector {
    pub fn new() -> Self {
        Self {
            stats: GlobalStats::new(),
        }
    }

    pub fn detect(&mut self, html: &str) -> Vec<Candidate> {
        let elements = HtmlParser::parse(html);
        let mut candidates = Vec::new();

        for elem in &elements {
            self.stats.record_element(elem);
        }

        for elem in elements {
            if Heuristic::should_fragment(&elem, &self.stats) {
                if let Some(candidate) = self.build_candidate(&elem) {
                    candidates.push(candidate);
                }
            }
        }

        self.deduplicate_candidates(candidates)
    }

    fn build_candidate(&self, elem: &ElementInfo) -> Option<Candidate> {
        let selector = if let Some(id) = &elem.id {
            format!("#{}", id)
        } else if let Some(class) = &elem.class {
            format!(".{}", class)
        } else {
            elem.tag.clone()
        };

        let id = if let Some(id) = &elem.id {
            id.clone()
        } else {
            format!("{}_{}", elem.tag, elem.depth)
        };

        Some(Candidate::new(id, selector, elem.outer_html.clone()))
    }

    fn deduplicate_candidates(&self, candidates: Vec<Candidate>) -> Vec<Candidate> {
        let mut seen = HashSet::new();
        let mut unique = Vec::new();
        for c in candidates {
            if !seen.contains(&c.id) {
                seen.insert(c.id.clone());
                unique.push(c);
            }
        }
        unique
    }
}