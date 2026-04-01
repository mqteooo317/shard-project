use shard::fragment::analyzer::BoundaryDetector;

#[test]
fn test_detect_candidates_by_id() {
    let html = r#"
    <html>
        <div id="header">Header content</div>
        <div>Regular div</div>
        <div id="footer">Footer content</div>
    </html>
    "#;
    let mut detector = BoundaryDetector::new();
    let candidates = detector.detect(html);
    assert_eq!(candidates.len(), 2);
    assert!(candidates.iter().any(|c| c.id == "header"));
    assert!(candidates.iter().any(|c| c.id == "footer"));
}

#[test]
fn test_detect_candidates_by_class() {
    let html = r#"
    <div class="sidebar">Sidebar</div>
    <div class="content">Main content</div>
    "#;
    let mut detector = BoundaryDetector::new();
    let candidates = detector.detect(html);
    assert_eq!(candidates.len(), 1);
    assert!(candidates[0].selector.contains("sidebar"));
}

#[test]
fn test_does_not_detect_small_elements() {
    let html = r#"<span>small</span>"#;
    let mut detector = BoundaryDetector::new();
    let candidates = detector.detect(html);
    assert_eq!(candidates.len(), 0);
}

#[test]
fn test_duplicate_candidates_deduplicated() {
    let html = r#"
    <div id="header">Header</div>
    <div id="header">Duplicate header</div>
    "#;
    let mut detector = BoundaryDetector::new();
    let candidates = detector.detect(html);
    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].id, "header");
}