use radiant_core::{
    RadiantRectangleNode, RadiantSceneMessage, RadiantSceneResponse, Runtime, SelectionTool,
};
use radiant_image_node::RadiantImageNode;
use radiant_text_node::RadiantTextNode;
use radiant_winit::RadiantView;

use crate::{RadiantMessage, RadiantNodeType, RadiantResponse};

pub struct RadiantRuntime {
    pub view: RadiantView<RadiantMessage, RadiantNodeType>,
}

impl RadiantRuntime {
    pub async fn new() -> Self {
        Self {
            view: RadiantView::new(SelectionTool::new()).await,
        }
    }
}

impl Runtime<'_, RadiantMessage, RadiantNodeType, RadiantResponse> for RadiantRuntime {
    type View = RadiantView<RadiantMessage, RadiantNodeType>;

    fn view(&self) -> &RadiantView<RadiantMessage, RadiantNodeType> {
        &self.view
    }

    fn view_mut(&mut self) -> &mut RadiantView<RadiantMessage, RadiantNodeType> {
        &mut self.view
    }

    fn handle_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse> {
        match message {
            RadiantMessage::SceneMessage(message) => {
                if let Some(response) = self.view.scene.handle_message(message) {
                    match response {
                        RadiantSceneResponse::Message { message } => {
                            return self.handle_message(message.into())
                        }
                        _ => return Some(response.into()),
                    }
                }
            }
            RadiantMessage::AddRectangle { position, scale } => {
                let id = self.view.scene.document.counter;
                let node = RadiantRectangleNode::new(id, position, scale);
                self.view.scene.add(node.into());
                return self
                    .handle_message(RadiantSceneMessage::SelectNode { id: Some(id) }.into());
            }
            RadiantMessage::AddImage { .. } => {
                let image = epaint::ColorImage::new([400, 100], epaint::Color32::RED);
                let texture_handle =
                    self.view
                        .scene
                        .texture_manager
                        .load_texture("test", image, Default::default());

                let id = self.view.scene.document.counter;
                let node =
                    RadiantImageNode::new(id, [400.0, 100.0], [100.0, 100.0], texture_handle);
                self.view.scene.add(node.into());
                return self
                    .handle_message(RadiantSceneMessage::SelectNode { id: Some(id) }.into());
            }
            RadiantMessage::AddText { position, .. } => {
                let id = self.view.scene.document.counter;
                let node = RadiantTextNode::new(id, position, [100.0, 100.0]);
                self.view.scene.add(node.into());
                return self
                    .handle_message(RadiantSceneMessage::SelectNode { id: Some(id) }.into());
            }
        }
        None
    }
}
