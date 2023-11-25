use radiantkit_core::{
    RadiantRectangleNode, RadiantSceneMessage, RadiantSceneResponse, RadiantTessellatable,
    RectangleTool, Runtime, Vec3, View,
};
use radiantkit_image::{image_loader, RadiantImageNode};
use radiantkit_text::RadiantTextNode;
use radiantkit_winit::RadiantView;
use radiankit_collaboration::Collaborator;
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
        if let Ok(mut document) = view.scene_mut().document.write() {
            document.add_listener(Box::new(Collaborator::new()));
        }
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
            RadiantMessage::AddImage { path, name } => {
                let screen_descriptor = self.view.scene().screen_descriptor;
                let texture_manager = self.view.scene_mut().texture_manager.clone();
                let document = self.view.scene_mut().document.clone();
                image_loader::load_image(path, move |response| {
                    let image = response
                        .unwrap_or(epaint::ColorImage::new([400, 100], epaint::Color32::RED));
                    let size = image.size;
                    if let Ok(mut document) = document.write() {
                        let texture_handle =
                            texture_manager.load_texture(name, image, Default::default());
                        let id = document.counter;
                        let mut node = RadiantImageNode::new(
                            id,
                            [100.0, 200.0],
                            [size[0] as f32, size[1] as f32],
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
            #[cfg(all(not(target_arch = "wasm32"), feature = "video"))]
            RadiantMessage::AddVideo { name, path } => {
                let screen_descriptor = self.view.scene().screen_descriptor;
                let texture_handle = self.view.scene_mut().texture_manager.load_texture(name, epaint::ColorImage::example(), Default::default());   
                if let Ok(mut document) = self.view.scene_mut().document.write() {
                    let id = document.counter;
                    println!("Adding video node with id: {}", id);
                    let mut node = radiantkit_video::RadiantVideoNode::new(
                        id,
                        [100.0, 200.0],
                        [100.0, 100.0],
                        path,
                        texture_handle,
                    );
                    node.attach(&screen_descriptor);
                    document.add(node.into());
                }
            }
            #[cfg(all(not(target_arch = "wasm32"), feature = "video"))]
            RadiantMessage::PlayVideo { id } => {
                if let Ok(mut document) = self.view.scene_mut().document.write() {
                    if let Some(RadiantNodeType::Video(video_node)) = document.get_node_mut(id) {
                        video_node.play();
                    }
                }
            }
        }
        None
    }
}
