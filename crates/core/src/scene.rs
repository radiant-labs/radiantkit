use epaint::{text::FontDefinitions, ClippedPrimitive, Fonts, TextureId};
use crate::{
    RadiantInteractionManager, RadiantRenderManager,
    RadiantTextureManager, RadiantTessellatable, RadiantDocumentNode,
    RadiantToolManager, ScreenDescriptor, RadiantNode, RadiantTool, InteractionMessage,
};

pub struct RadiantScene<M, N: RadiantNode> {
    pub document: RadiantDocumentNode<N>,

    pub screen_descriptor: ScreenDescriptor,

    pub fonts_manager: epaint::Fonts,
    pub render_manager: RadiantRenderManager,
    pub tool_manager: RadiantToolManager<M>,
    pub interaction_manager: RadiantInteractionManager<M>,
    pub texture_manager: RadiantTextureManager,
}

impl<M: From<InteractionMessage> + TryInto<InteractionMessage>, N: RadiantNode> RadiantScene<M, N> {
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
            tool_manager: RadiantToolManager::new(Box::new(default_tool)),
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
