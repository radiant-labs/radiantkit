use radiant_core::RadiantDocumentNode;
use radiant_scene::RadiantNodeType;

#[test]
fn test_add_artboard() {
    let mut document = RadiantDocumentNode::<RadiantNodeType>::new();
    document.add_artboard();

    assert_eq!(document.artboards.len(), 2);
}
