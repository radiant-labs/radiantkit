use radiant_core::{RadiantDocumentNode, RadiantMessageHandler, RadiantDocumentMessage};

#[test]
fn test_messages() {
    let mut document = RadiantDocumentNode::new();
    document.handle_message(RadiantDocumentMessage::AddArtboard);

    assert_eq!(document.artboards.len(), 2);
}