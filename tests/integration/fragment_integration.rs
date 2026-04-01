use shard::fragment::{BoundaryDetector, FragmentStore, Merger, TemplateCache, DependencyGraph};
use shard::fragment::types::Fragment;

#[test]
fn test_fragment_store_and_reassemble() {
    let html = r#"
    <html>
        <div id="header">Header content</div>
        <div id="content">Dynamic content</div>
        <div id="footer">Footer content</div>
    </html>
    "#;
    let mut detector = BoundaryDetector::new();
    let candidates = detector.detect(html);

    let store = FragmentStore::new();
    let template_cache = TemplateCache::new();
    let mut replacements = Vec::new();

    for cand in candidates {
        let fragment = Fragment::new(cand.id.clone(), cand.content.clone());
        store.set(cand.id.clone(), fragment);
        replacements.push((cand.content.clone(), shard::fragment::Placeholder::new(&cand.id)));
    }

    let mut template = html.to_string();
    for (content, placeholder) in replacements {
        template = template.replace(&content, &placeholder.to_marker());
    }
    template_cache.set("/test".to_string(), template.clone());

    // Simulate later: rebuild from fragments
    let template = template_cache.get("/test").unwrap();
    let mut fragments = std::collections::HashMap::new();
    fragments.insert("header".to_string(), store.get("header").unwrap().content);
    fragments.insert("footer".to_string(), store.get("footer").unwrap().content);
    fragments.insert("content".to_string(), "<div id=\"content\">New dynamic content</div>".to_string());

    let rebuilt = Merger::merge(&template, &fragments);
    assert!(rebuilt.contains("New dynamic content"));
    assert!(rebuilt.contains("Header content"));
    assert!(rebuilt.contains("Footer content"));
}