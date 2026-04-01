use shard::cache::l1::ShardCache;

#[test]
fn test_set_and_get() {
    let cache = ShardCache::new(4, 1024, 60);
    cache.set_str("key1", vec![1, 2, 3]);
    let val = cache.get_str("key1");
    assert_eq!(val, Some(vec![1, 2, 3]));
}

#[test]
fn test_ttl_expiration() {
    let cache = ShardCache::new(4, 1024, 1);
    cache.set_str("key1", vec![1, 2, 3]);
    std::thread::sleep(std::time::Duration::from_secs(2));
    cache.evict_expired();
    let val = cache.get_str("key1");
    assert_eq!(val, None);
}

#[test]
fn test_remove() {
    let cache = ShardCache::new(4, 1024, 60);
    cache.set_str("key1", vec![1, 2, 3]);
    cache.remove_str("key1");
    let val = cache.get_str("key1");
    assert_eq!(val, None);
}