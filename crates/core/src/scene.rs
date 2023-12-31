use std::sync::Arc;

use crate::{
    ColorComponent, RadiantDocumentNode, RadiantInteractionManager, RadiantNode,
    RadiantRenderManager, RadiantSceneMessage, RadiantSceneResponse, RadiantTessellatable,
    RadiantTextureManager, RadiantToolManager, ScreenDescriptor, SelectionTool, TransformComponent,
};
use epaint::{text::FontDefinitions, ClippedPrimitive, Fonts, TextureId};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

pub struct RadiantScene<M, N: RadiantNode> {
    pub document: Arc<RwLock<RadiantDocumentNode<N>>>,

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
    pub fn document(&self) -> RwLockReadGuard<RadiantDocumentNode<N>> {
        self.document.read()
    }

    pub fn document_mut(&mut self) -> RwLockWriteGuard<RadiantDocumentNode<N>> {
        self.document.write()
    }

    pub fn new(
        config: wgpu::SurfaceConfiguration,
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        screen_descriptor: ScreenDescriptor,
    ) -> Self {
        let font_definitions = FontDefinitions::default();
        let fonts_manager = Fonts::new(screen_descriptor.pixels_per_point, 1600, font_definitions);
        let texture_manager = RadiantTextureManager::default();
        let render_manager = RadiantRenderManager::new(config, surface, device, queue, None);

        Self {
            document: Arc::new(RwLock::new(RadiantDocumentNode::new())),

            screen_descriptor,

            fonts_manager,
            render_manager,
            tool_manager: RadiantToolManager::new(0u32, Box::new(SelectionTool::new())),
            interaction_manager: RadiantInteractionManager::new(),
            texture_manager,
        }
    }

    pub fn add(&mut self, mut node: N) {
        node.attach(&self.screen_descriptor);
        self.document_mut().add(node);
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

    pub async fn select(&mut self, mouse_position: [f32; 2]) -> Option<Uuid> {
        let primitives = self.get_primitives(true);
        self.render_manager
            .render_offscreen(primitives, &self.screen_descriptor, true, mouse_position)
            .await
            .unwrap()
    }

    fn get_primitives(&mut self, selection: bool) -> Vec<ClippedPrimitive> {
        let mut primitives = self.document.write().tessellate(
            selection,
            &self.screen_descriptor,
            &self.fonts_manager,
        );

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
                self.document_mut().add_artboard();
            }
            RadiantSceneMessage::SelectArtboard { id } => {
                self.document_mut().set_active_artboard(id);
            }
            RadiantSceneMessage::SelectNode { id } => {
                self.document_mut().select(id);
                if let Some(id) = id {
                    if !self.interaction_manager.is_interaction(id) {
                        if let Some(mut node) = self.document.write().get_node_mut(id) {
                            node.tessellate(false, &self.screen_descriptor, &self.fonts_manager);
                            let response = RadiantSceneResponse::Selected { node: node.clone() };
                            self.interaction_manager
                                .enable_interactions(node, &self.screen_descriptor);
                            return Some(response);
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
                } else if let Some(mut node) = self.document.write().get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<TransformComponent>() {
                        component.transform_xy(&position.into());
                        component.transform_scale(&scale.into());

                        let response = RadiantSceneResponse::TransformUpdated {
                            id,
                            position: component.position().into(),
                            scale: component.scale().into(),
                        };

                        node.set_needs_tessellation(true);
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
                if let Some(mut node) = self.document.write().get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<TransformComponent>() {
                        component.set_position(&position.into());
                        component.set_scale(&scale.into());
                        node.set_needs_tessellation(true);

                        self.interaction_manager
                            .update_interactions(node, &self.screen_descriptor);
                    }
                }
            }
            RadiantSceneMessage::SetFillColor { id, fill_color } => {
                if let Some(mut node) = self.document_mut().get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<ColorComponent>() {
                        component.set_fill_color(fill_color);
                        node.set_needs_tessellation(true);
                    }
                }
            }
            RadiantSceneMessage::SetStrokeColor { id, stroke_color } => {
                if let Some(mut node) = self.document_mut().get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<ColorComponent>() {
                        component.set_stroke_color(stroke_color);
                        node.set_needs_tessellation(true);
                    }
                }
            }
            RadiantSceneMessage::SelectTool { id } => {
                self.tool_manager.activate_tool(id);
            }
            RadiantSceneMessage::HandleKey { id, key } => {
                if let Some(id) = match id {
                    Some(id) => Some(id),
                    None => self.document.read().selected_node_id,
                } {
                    if let Some(mut node) = self.document.write().get_node_mut(id) {
                        if node.handle_key_down(key) {
                            self.interaction_manager
                                .update_interactions(node, &self.screen_descriptor);
                        }
                    }
                }
            }
        }
        None
    }
}
