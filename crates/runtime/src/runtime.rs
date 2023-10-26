use radiant_core::{
    ColorComponent, RadiantComponentProvider, RadiantRectangleNode, RadiantTessellatable,
    RadiantTransformable, SelectionTool, TransformComponent,
};
use radiant_image_node::RadiantImageNode;
use radiant_text_node::RadiantTextNode;
use radiant_winit::{RadiantApp, Runtime};

use crate::{RadiantMessage, RadiantNodeType, RadiantResponse};

pub struct RadiantRuntime {
    pub app: RadiantApp<RadiantMessage, RadiantNodeType>,
    pub handler: Box<dyn Fn(RadiantResponse)>,
}

impl RadiantRuntime {
    pub async fn new(handler: Box<dyn Fn(RadiantResponse)>) -> Self {
        Self {
            app: RadiantApp::new(SelectionTool::new()).await,
            handler,
        }
    }

    pub fn handle_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse> {
        match message {
            RadiantMessage::AddArtboard => {
                self.app.scene.document.add_artboard();
            }
            RadiantMessage::SelectArtboard(id) => {
                self.app.scene.document.set_active_artboard(id);
            }
            RadiantMessage::SelectNode(id) => {
                if !self.app.scene.interaction_manager.is_interaction(id) {
                    self.app.scene.document.select(id);
                    if let Some(node) = self.app.scene.document.get_node(id) {
                        self.app
                            .scene
                            .interaction_manager
                            .enable_interactions(node, &self.app.scene.screen_descriptor);
                        return Some(RadiantResponse::NodeSelected(node.clone()));
                    } else {
                        self.app.scene.interaction_manager.disable_interactions();
                    }
                }
            }
            RadiantMessage::AddNode {
                node_type,
                position,
                scale,
            } => {
                let id = self.app.scene.document.counter;
                let node = match node_type.as_str() {
                    "Rectangle" => Some(RadiantNodeType::Rectangle(RadiantRectangleNode::new(
                        id, position, scale,
                    ))),
                    _ => None,
                };
                if let Some(node) = node {
                    self.app.scene.add(node);
                    return self.handle_message(RadiantMessage::SelectNode(id));
                }
            }
            RadiantMessage::TransformNode {
                id,
                position,
                scale,
            } => {
                if self.app.scene.interaction_manager.is_interaction(id) {
                    if let Some(message) = self
                        .app
                        .scene
                        .interaction_manager
                        .handle_interaction(message)
                    {
                        return self.handle_message(message);
                    }
                } else if let Some(node) = self.app.scene.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<TransformComponent>() {
                        component.transform_xy(&position);
                        component.transform_scale(&scale);

                        let response = RadiantResponse::TransformUpdated {
                            id,
                            position: component.get_xy(),
                            scale: component.get_scale(),
                        };

                        node.set_needs_tessellation();
                        self.app
                            .scene
                            .interaction_manager
                            .update_interactions(node, &self.app.scene.screen_descriptor);

                        return Some(response);
                    }
                }
            }
            RadiantMessage::SetTransform {
                id,
                position,
                scale,
            } => {
                if let Some(node) = self.app.scene.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<TransformComponent>() {
                        component.set_xy(&position);
                        component.set_scale(&scale);
                        node.set_needs_tessellation();

                        self.app
                            .scene
                            .interaction_manager
                            .update_interactions(node, &self.app.scene.screen_descriptor);
                    }
                }
            }
            RadiantMessage::SetFillColor { id, fill_color } => {
                if let Some(node) = self.app.scene.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<ColorComponent>() {
                        component.set_fill_color(fill_color);
                        node.set_needs_tessellation();
                    }
                }
            }
            RadiantMessage::SetStrokeColor { id, stroke_color } => {
                if let Some(node) = self.app.scene.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<ColorComponent>() {
                        component.set_stroke_color(stroke_color);
                        node.set_needs_tessellation();
                    }
                }
            }
            RadiantMessage::SelectTool { id } => {
                self.app.scene.tool_manager.activate_tool(id);
            }
            RadiantMessage::AddImage { .. } => {
                let image = epaint::ColorImage::new([400, 100], epaint::Color32::RED);
                let texture_handle =
                    self.app
                        .scene
                        .texture_manager
                        .load_texture("test", image, Default::default());

                let id = self.app.scene.document.counter;
                let node = RadiantNodeType::Image(RadiantImageNode::new(
                    id,
                    [400.0, 100.0],
                    [100.0, 100.0],
                    texture_handle,
                ));
                self.app.scene.add(node);
                return self.handle_message(RadiantMessage::SelectNode(id));
            }
            RadiantMessage::AddText { position, .. } => {
                let id = self.app.scene.document.counter;
                let node =
                    RadiantNodeType::Text(RadiantTextNode::new(id, position, [100.0, 100.0]));
                self.app.scene.add(node);
                return self.handle_message(RadiantMessage::SelectNode(id));
            }
        }
        None
    }
}

impl RadiantRuntime {
    pub fn process_message(&mut self, message: RadiantMessage) {
        let response = self.handle_message(message);
        self.handle_response(response);
    }

    fn handle_response(&self, response: Option<RadiantResponse>) {
        if let Some(response) = response {
            (self.handler)(response);
        }
    }
}

impl Runtime<RadiantMessage, RadiantNodeType, RadiantResponse> for RadiantRuntime {
    fn app(&mut self) -> &mut RadiantApp<RadiantMessage, RadiantNodeType> {
        &mut self.app
    }

    fn handle_runtime_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse> {
        self.handle_message(message)
    }
}
