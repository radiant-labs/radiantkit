use crate::{
    ColorComponent, RadiantDocumentNode, RadiantInteractionManager, RadiantNode,
    RadiantRenderManager, RadiantSceneMessage, RadiantSceneResponse, RadiantTessellatable,
    RadiantTextureManager, RadiantTool, RadiantToolManager, RadiantTransformable, ScreenDescriptor,
    TransformComponent,
};
use epaint::{text::FontDefinitions, ClippedPrimitive, Fonts, TextureId};

pub struct RadiantScene<M, N: RadiantNode> {
    pub document: RadiantDocumentNode<N>,

    pub screen_descriptor: ScreenDescriptor,

    pub fonts_manager: epaint::Fonts,
    pub render_manager: RadiantRenderManager,
    pub tool_manager: RadiantToolManager<M>,
    pub interaction_manager: RadiantInteractionManager<M>,
    pub texture_manager: RadiantTextureManager,
}

impl<M: From<RadiantSceneMessage> + TryInto<RadiantSceneMessage>, N: RadiantNode>
    RadiantScene<M, N>
{
    pub fn new(
        config: wgpu::SurfaceConfiguration,
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        screen_descriptor: ScreenDescriptor,
        default_tool: impl RadiantTool<M> + 'static,
    ) -> Self {
        let font_definitions = FontDefinitions::default();
        let fonts_manager = Fonts::new(screen_descriptor.pixels_per_point, 1600, font_definitions);
        let texture_manager = RadiantTextureManager::default();
        let render_manager = RadiantRenderManager::new(config, surface, device, queue, None);

        // let mut tool_manager = RadiantToolManager::new(Box::new(SelectionTool::new()));
        // tool_manager.register_tool(Box::new(PathTool::new()));

        Self {
            document: RadiantDocumentNode::new(),

            screen_descriptor,

            fonts_manager,
            render_manager,
            tool_manager: RadiantToolManager::new(0u32, Box::new(default_tool)),
            interaction_manager: RadiantInteractionManager::new(),
            texture_manager,
        }
    }

    pub fn add(&mut self, mut node: N) {
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
}

impl<M: From<RadiantSceneMessage> + TryInto<RadiantSceneMessage>, N: RadiantNode>
    RadiantScene<M, N>
{
    pub fn handle_message(
        &mut self,
        message: RadiantSceneMessage,
    ) -> Option<RadiantSceneResponse<M, N>> {
        match message {
            RadiantSceneMessage::AddArtboard {} => {
                self.document.add_artboard();
            }
            RadiantSceneMessage::SelectArtboard { id } => {
                self.document.set_active_artboard(id);
            }
            RadiantSceneMessage::SelectNode { id } => {
                self.document.select(id);
                if let Some(id) = id {
                    if !self.interaction_manager.is_interaction(id) {
                        if let Some(node) = self.document.get_node(id) {
                            self.interaction_manager
                                .enable_interactions(node, &self.screen_descriptor);
                            return Some(RadiantSceneResponse::Selected { node: node.clone() });
                        } else {
                            self.interaction_manager.disable_interactions();
                        }
                    }
                } else {
                    self.interaction_manager.disable_interactions();
                }
            }
            RadiantSceneMessage::TransformNode {
                id,
                position,
                scale,
            } => {
                if self.interaction_manager.is_interaction(id) {
                    if let Some(message) =
                        self.interaction_manager.handle_interaction(message.into())
                    {
                        return Some(RadiantSceneResponse::Message { message });
                    }
                } else if let Some(node) = self.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<TransformComponent>() {
                        component.transform_xy(&position);
                        component.transform_scale(&scale);

                        let response = RadiantSceneResponse::TransformUpdated {
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
            RadiantSceneMessage::SetTransform {
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
            RadiantSceneMessage::SetFillColor { id, fill_color } => {
                if let Some(node) = self.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<ColorComponent>() {
                        component.set_fill_color(fill_color);
                        node.set_needs_tessellation();
                    }
                }
            }
            RadiantSceneMessage::SetStrokeColor { id, stroke_color } => {
                if let Some(node) = self.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<ColorComponent>() {
                        component.set_stroke_color(stroke_color);
                        node.set_needs_tessellation();
                    }
                }
            }
            RadiantSceneMessage::SelectTool { id } => {
                self.tool_manager.activate_tool(id);
            }
        }
        None
    }
}
