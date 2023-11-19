use radiantkit_core::{
    RadiantRectangleNode, RadiantSceneMessage, RadiantSceneResponse, RadiantTessellatable,
    RectangleTool, Runtime, Vec3, View,
};
use radiantkit_image::{image_loader, RadiantImageNode};
use radiantkit_text::RadiantTextNode;
use radiantkit_winit::RadiantView;

use crate::{RadiantMessage, RadiantNodeType, RadiantResponse, RadiantToolType};

pub struct RadiantRuntime {
    pub view: RadiantView<RadiantMessage, RadiantNodeType>,
}

impl RadiantRuntime {
    pub async fn new(size: Option<Vec3>) -> Self {
        let mut view = RadiantView::new(size).await;
        view.scene_mut().tool_manager.register_tool(
            RadiantToolType::Rectangle as u32,
            Box::new(RectangleTool::new()),
        );
        Self { view }
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
                let response = self.view.scene_mut().handle_message(message);
                if let Some(response) = response {
                    match response {
                        RadiantSceneResponse::Message { message } => {
                            return self.handle_message(message.into())
                        }
                        _ => return Some(response.into()),
                    }
                }
            }
            RadiantMessage::AddRectangle { position, scale } => {
                let id = self.view.scene().document().counter;
                let node = RadiantRectangleNode::new(id, position, scale);
                self.view.scene_mut().add(node.into());
                return self
                    .handle_message(RadiantSceneMessage::SelectNode { id: Some(id) }.into());
            }
            RadiantMessage::AddImage { path, .. } => {
                let screen_descriptor = self.view.scene().screen_descriptor;
                let texture_manager = self.view.scene_mut().texture_manager.clone();
                let document = self.view.scene_mut().document.clone();
                image_loader::load_image(path, move |response| {
                    let image = response
                        .unwrap_or(epaint::ColorImage::new([400, 100], epaint::Color32::RED));
                    if let Ok(mut document) = document.write() {
                        let texture_handle =
                            texture_manager.load_texture("test", image, Default::default());
                        let id = document.counter;
                        let mut node = RadiantImageNode::new(
                            id,
                            [100.0, 200.0],
                            [100.0, 100.0],
                            texture_handle,
                        );
                        node.attach(&screen_descriptor);
                        document.add(node.into());
                    }
                });
            }
            RadiantMessage::AddText { text, position } => {
                let id = self.view.scene().document().counter;
                let node = RadiantTextNode::new(id, text, position, [100.0, 100.0]);
                self.view.scene_mut().add(node.into());
                return self
                    .handle_message(RadiantSceneMessage::SelectNode { id: Some(id) }.into());
            }
        }
        None
    }
}
