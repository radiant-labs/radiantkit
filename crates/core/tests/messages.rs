use radiant_core::RadiantDocumentNode;

#[test]
fn test_add_artboard() {
    let mut document = RadiantDocumentNode::new();
    document.add_artboard();

    assert_eq!(document.artboards.len(), 2);
}
