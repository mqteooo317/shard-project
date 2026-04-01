use shard::cache::multi_level::MultiLevelCache;
use tempfile::tempdir;

#[tokio::test]
async fn test_multi_level_get_set() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_str().unwrap();
    let cache = MultiLevelCache::new(1024, 60, path, 60);

    cache.set("key1", vec![1, 2, 3]).await;
    let val = cache.get("key1").await;
    assert_eq!(val, Some(vec![1, 2, 3]));

    // L2 persistence: create new cache with same path
    let cache2 = MultiLevelCache::new(1024, 60, path, 60);
    let val2 = cache2.get("key1").await;
    assert_eq!(val2, Some(vec![1, 2, 3]));
}