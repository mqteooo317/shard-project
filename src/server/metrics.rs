use prometheus::{register_counter, register_histogram, Counter, Histogram};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref REQUESTS_TOTAL: Counter = register_counter!("shard_requests_total", "Total requests").unwrap();
    pub static ref CACHE_HITS: Counter = register_counter!("shard_cache_hits", "Cache hits").unwrap();
    pub static ref CACHE_MISSES: Counter = register_counter!("shard_cache_misses", "Cache misses").unwrap();
    pub static ref REQUEST_DURATION: Histogram = register_histogram!("shard_request_duration_seconds", "Request duration").unwrap();
}