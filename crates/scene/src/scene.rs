use epaint::{text::FontDefinitions, ClippedPrimitive, Fonts, TextureId};
use radiant_path_node::PathTool;
use serde::{Deserialize, Serialize};
use radiant_core::{
    ColorComponent,
    RadiantInteractionManager, RadiantRenderManager,
     RadiantTextureManager, RadiantTessellatable, RadiantComponentProvider, RadiantTransformable, RadiantDocumentNode,
    RadiantToolManager, TransformComponent, ScreenDescriptor, RadiantRectangleNode,
};
use radiant_image_node::RadiantImageNode;
use radiant_text_node::RadiantTextNode;
use crate::{RadiantNodeType, RadiantMessage};

pub struct RadiantScene {
    pub document: RadiantDocumentNode<RadiantNodeType>,
    pub handler: Box<dyn Fn(RadiantResponse)>,

    pub screen_descriptor: ScreenDescriptor,

    pub fonts_manager: epaint::Fonts,
    pub render_manager: RadiantRenderManager,
    tool_manager: RadiantToolManager<RadiantMessage>,
    interaction_manager: RadiantInteractionManager<RadiantMessage>,
    texture_manager: RadiantTextureManager,
}

impl RadiantScene {
    pub fn new(
        config: wgpu::SurfaceConfiguration,
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        screen_descriptor: ScreenDescriptor,
        handler: Box<dyn Fn(RadiantResponse)>,
    ) -> Self {
        let font_definitions = FontDefinitions::default();
        let fonts_manager = Fonts::new(screen_descriptor.pixels_per_point, 1600, font_definitions);
        let texture_manager = RadiantTextureManager::default();
        let render_manager = RadiantRenderManager::new(config, surface, device, queue, None);

        let mut tool_manager = RadiantToolManager::new();
        tool_manager.register_tool(Box::new(PathTool::new()));

        Self {
            document: RadiantDocumentNode::new(),
            handler,

            screen_descriptor,

            fonts_manager,
            render_manager,
            tool_manager,
            interaction_manager: RadiantInteractionManager::new(),
            texture_manager,
        }
    }
}

impl RadiantScene {
    pub fn add(&mut self, mut node: RadiantNodeType) {
        node.attach(&self.screen_descriptor);
        self.document.add(node);
    }

    pub fn resize(&mut self, new_size: [u32; 2]) {
        if new_size[0] > 0 && new_size[1] > 0 {
            self.screen_descriptor.size_in_pixels = new_size;
            self.render_manager.resize(new_size);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.fonts_manager
            .begin_frame(self.screen_descriptor.pixels_per_point, 1024);
        if let Some(font_image_delta) = self.fonts_manager.font_image_delta() {
            self.texture_manager
                .0
                .write()
                .set(TextureId::default(), font_image_delta);
        }

        let delta = self.texture_manager.0.write().take_delta();
        self.render_manager.update_textures(delta);

        let primitives = self.get_primitives(false);
        self.render_manager
            .render(primitives, &self.screen_descriptor, false)
    }

    pub async fn select(&mut self, mouse_position: [f32; 2]) -> u64 {
        let primitives = self.get_primitives(true);
        self.render_manager
            .render_offscreen(primitives, &self.screen_descriptor, true, mouse_position)
            .await
            .unwrap()
    }

    fn get_primitives(&mut self, selection: bool) -> Vec<ClippedPrimitive> {
        let mut primitives =
            self.document
                .tessellate(selection, &self.screen_descriptor, &self.fonts_manager);

        let mut p2 = self.interaction_manager.tessellate(
            selection,
            &self.screen_descriptor,
            &self.fonts_manager,
        );
        primitives.append(&mut p2);

        primitives
    }

    fn handle_response(&self, response: Option<RadiantResponse>) {
        if let Some(response) = response {
            (self.handler)(response);
        }
    }
}

impl RadiantScene {
    pub fn handle_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse> {
        match message {
            RadiantMessage::AddArtboard => {
                self.document.add_artboard();
            }
            RadiantMessage::SelectArtboard(id) => {
                self.document.set_active_artboard(id);
            }
            RadiantMessage::SelectNode(id) => {
                if !self.interaction_manager.is_interaction(id) {
                    self.document.select(id);
                    if let Some(node) = self.document.get_node(id) {
                        self.interaction_manager
                            .enable_interactions(node, &self.screen_descriptor);
                        return Some(RadiantResponse::NodeSelected(node.clone()));
                    } else {
                        self.interaction_manager.disable_interactions();
                    }
                }
            }
            RadiantMessage::AddNode {
                node_type,
                position,
                scale,
            } => {
                let id = self.document.counter;
                let node = match node_type.as_str() {
                    "Rectangle" =>
                        Some(RadiantNodeType::Rectangle(RadiantRectangleNode::new(
                            id,
                            position,
                            scale,
                        ))),
                    _ => None
                };
                if let Some(node) = node {
                    self.add(node);
                    return self.handle_message(RadiantMessage::SelectNode(id));
                }
            }
            RadiantMessage::TransformNode {
                id,
                position,
                scale,
            } => {
                if self.interaction_manager.is_interaction(id) {
                    if let Some(message) = self.interaction_manager.handle_interaction(message) {
                        return self.handle_message(message);
                    }
                } else if let Some(node) = self.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<TransformComponent>() {
                        component.transform_xy(&position);
                        component.transform_scale(&scale);

                        let response = RadiantResponse::TransformUpdated {
                            id,
                            position: component.get_xy(),
                            scale: component.get_scale(),
                        };

                        node.set_needs_tessellation();
                        self.interaction_manager
                            .update_interactions(node, &self.screen_descriptor);

                        return Some(response);
                    }
                }
            }
            RadiantMessage::SetTransform {
                id,
                position,
                scale,
            } => {
                if let Some(node) = self.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<TransformComponent>() {
                        component.set_xy(&position);
                        component.set_scale(&scale);
                        node.set_needs_tessellation();

                        self.interaction_manager
                            .update_interactions(node, &self.screen_descriptor);
                    }
                }
            }
            RadiantMessage::SetFillColor { id, fill_color } => {
                if let Some(node) = self.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<ColorComponent>() {
                        component.set_fill_color(fill_color);
                        node.set_needs_tessellation();
                    }
                }
            }
            RadiantMessage::SetStrokeColor { id, stroke_color } => {
                if let Some(node) = self.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<ColorComponent>() {
                        component.set_stroke_color(stroke_color);
                        node.set_needs_tessellation();
                    }
                }
            }
            RadiantMessage::SelectTool { id } => {
                self.tool_manager.activate_tool(id);
            }
            RadiantMessage::AddImage { .. } => {
                let image = epaint::ColorImage::new([400, 100], epaint::Color32::RED);
                let texture_handle =
                    self.texture_manager
                        .load_texture("test", image, Default::default());

                let id = self.document.counter;
                let node = RadiantNodeType::Image(RadiantImageNode::new(
                    id,
                    [400.0, 100.0],
                    [100.0, 100.0],
                    texture_handle,
                ));
                self.add(node);
                return self.handle_message(RadiantMessage::SelectNode(id));
            }
            RadiantMessage::AddText { position, .. } => {
                let id = self.document.counter;
                let node =
                    RadiantNodeType::Text(RadiantTextNode::new(id, position, [100.0, 100.0]));
                self.add(node);
                return self.handle_message(RadiantMessage::SelectNode(id));
            }
        }
        None
    }
}

impl RadiantScene {
    pub fn process_message(&mut self, message: RadiantMessage) {
        let response = self.handle_message(message);
        self.handle_response(response);
    }
}

impl RadiantScene {
    pub fn on_mouse_down(&mut self, position: [f32; 2]) -> bool {
        let id = pollster::block_on(self.select(position));
        if let Some(message) =
            self.tool_manager
                .active_tool()
                .on_mouse_down(id,  position)
        {
            self.process_message(message);
            true
        } else {
            false
        }
    }

    pub fn on_mouse_move(&mut self, position: [f32; 2]) -> bool {
        if let Some(message) = self
            .tool_manager
            .active_tool()
            .on_mouse_move(position)
        {
            self.process_message(message);
            true
        } else {
            false
        }
    }

    pub fn on_mouse_up(&mut self, position: [f32; 2]) -> bool {
        if let Some(message) = self
            .tool_manager
            .active_tool()
            .on_mouse_up(position)
        {
            self.process_message(message);
            true
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantResponse {
    NodeSelected(RadiantNodeType),
    TransformUpdated {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },
}
