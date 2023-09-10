use radiant_core::{RadiantMessage, RadiantMessageHandler, RadiantDocumentNode};

#[test]
fn test_messages() {
    let mut document = RadiantDocumentNode::new();
    document.handle_message(RadiantMessage::AddArtboard);

    assert_eq!(document.artboards.len(), 2);
}