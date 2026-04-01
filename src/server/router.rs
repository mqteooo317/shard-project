use crate::cache::l1::CacheKey;

pub struct Router;

impl Router {
    pub fn cache_key(&self, uri: &str, method: &str) -> Option<CacheKey> {
        // Only GET requests are cacheable by default
        if method != "GET" {
            return None;
        }

        // Exclude admin paths (configurable later)
        if uri.contains("/wp-admin") || uri.contains("/admin") {
            return None;
        }

        Some(CacheKey::new(uri.to_string()))
    }

    pub fn is_cacheable(&self, uri: &str, method: &str, headers: &hyper::HeaderMap) -> bool {
        if method != "GET" {
            return false;
        }

        // Check Cache-Control: no-cache
        if let Some(cc) = headers.get(hyper::header::CACHE_CONTROL) {
            if cc.to_str().unwrap_or("").contains("no-cache") {
                return false;
            }
        }

        // Exclude admin paths
        if uri.contains("/wp-admin") || uri.contains("/admin") {
            return false;
        }

        true
    }
}