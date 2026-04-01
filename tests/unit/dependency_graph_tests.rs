use shard::fragment::DependencyGraph;

#[test]
fn test_add_and_get_dependents() {
    let graph = DependencyGraph::new();
    graph.add_dependency("/page1".to_string(), "frag1".to_string());
    graph.add_dependency("/page1".to_string(), "frag2".to_string());
    graph.add_dependency("/page2".to_string(), "frag1".to_string());

    let dependents = graph.get_dependents("frag1");
    assert_eq!(dependents.len(), 2);
    assert!(dependents.contains("/page1"));
    assert!(dependents.contains("/page2"));

    let page_frags = graph.get_fragments_for_page("/page1");
    assert_eq!(page_frags.len(), 2);
    assert!(page_frags.contains("frag1"));
    assert!(page_frags.contains("frag2"));
}

#[test]
fn test_remove_page() {
    let graph = DependencyGraph::new();
    graph.add_dependency("/page1".to_string(), "frag1".to_string());
    graph.add_dependency("/page2".to_string(), "frag1".to_string());

    graph.remove_page("/page1");
    let dependents = graph.get_dependents("frag1");
    assert_eq!(dependents.len(), 1);
    assert!(dependents.contains("/page2"));
    assert!(!dependents.contains("/page1"));
}