use radiantkit_core::RadiantDocumentNode;
use radiantkit::RadiantNodeType;

#[test]
fn test_add_artboard() {
    let mut document = RadiantDocumentNode::<RadiantNodeType>::new();
    document.add_artboard();

    assert_eq!(document.artboards.len(), 2);
}
