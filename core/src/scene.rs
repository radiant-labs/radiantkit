use epaint::{text::FontDefinitions, ClippedPrimitive, Fonts, TextureId};

use crate::{
    RadiantComponentProvider, RadiantDocumentNode, RadiantImageNode, RadiantInteractionManager,
    RadiantMessage, RadiantNode, RadiantNodeType, RadiantRenderManager, RadiantResponse,
    RadiantTessellatable, RadiantTextureManager, RadiantToolManager, RadiantTransformable,
    TransformComponent, ColorComponent,
};

/// Information about the screen used for rendering.
pub struct ScreenDescriptor {
    /// Size of the window in physical pixels.
    pub size_in_pixels: [u32; 2],

    /// HiDPI scale factor (pixels per point).
    pub pixels_per_point: f32,
}

impl ScreenDescriptor {
    /// size in "logical" points
    pub fn screen_size_in_points(&self) -> [f32; 2] {
        [
            self.size_in_pixels[0] as f32 / self.pixels_per_point,
            self.size_in_pixels[1] as f32 / self.pixels_per_point,
        ]
    }
}

pub struct RadiantScene {
    pub document: RadiantDocumentNode,
    pub handler: Box<dyn Fn(RadiantResponse)>,

    pub screen_descriptor: ScreenDescriptor,

    pub fonts: epaint::Fonts,

    pub render_manager: RadiantRenderManager,
    tool_manager: RadiantToolManager,
    interaction_manager: RadiantInteractionManager,
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
        let fonts = Fonts::new(screen_descriptor.pixels_per_point, 1600, font_definitions);
        fonts.begin_frame(screen_descriptor.pixels_per_point, 1600);

        let texture_manager = RadiantTextureManager::default();

        if let Some(font_image_delta) = fonts.font_image_delta() {
            texture_manager
                .0
                .write()
                .set(TextureId::default(), font_image_delta);
        }

        let render_manager = RadiantRenderManager::new(config, surface, device, queue, None);

        Self {
            document: RadiantDocumentNode::new(),
            handler,

            screen_descriptor,
            fonts,

            render_manager,
            tool_manager: RadiantToolManager::new(),
            interaction_manager: RadiantInteractionManager::new(),
            texture_manager,
        }
    }
}

impl RadiantScene {
    pub fn add(&mut self, mut node: RadiantNodeType) {
        node.attach_to_scene(self);
        self.document.add(node);
    }

    pub fn resize(&mut self, new_size: [u32; 2]) {
        if new_size[0] > 0 && new_size[1] > 0 {
            self.screen_descriptor.size_in_pixels = new_size;
            self.render_manager.resize(new_size);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
        let mut primitives = self.document.tessellate(selection, &self.screen_descriptor);

        let mut p2 = self
            .interaction_manager
            .tessellate(selection, &self.screen_descriptor);
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
            RadiantMessage::AddNode(mut node) => {
                let id = node.get_id();
                if id == 0 {
                    node.set_id(id);
                }
                self.add(node);
                return self.handle_message(RadiantMessage::SelectNode(id));
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
            RadiantMessage::SelectTool(tool) => {
                self.tool_manager.activate_tool(tool);
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
                .active_tool
                .on_mouse_down(id, &self.document, position)
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
            .active_tool
            .on_mouse_move(&self.document, position)
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
            .active_tool
            .on_mouse_up(&self.document, position)
        {
            self.process_message(message);
            true
        } else {
            false
        }
    }
}
