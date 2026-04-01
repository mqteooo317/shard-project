use shard::fragment::FragmentStore;
use shard::fragment::types::Fragment;

#[test]
fn test_store_and_get() {
    let store = FragmentStore::new();
    let fragment = Fragment::new("test".to_string(), "<div>test</div>".to_string());
    store.set("key".to_string(), fragment.clone());
    let retrieved = store.get("key");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "test");
}

#[test]
fn test_remove() {
    let store = FragmentStore::new();
    let fragment = Fragment::new("test".to_string(), "content".to_string());
    store.set("key".to_string(), fragment);
    assert!(store.get("key").is_some());
    store.remove("key");
    assert!(store.get("key").is_none());
}

#[test]
fn test_contains() {
    let store = FragmentStore::new();
    assert!(!store.contains("key"));
    store.set("key".to_string(), Fragment::new("test".to_string(), "".to_string()));
    assert!(store.contains("key"));
}