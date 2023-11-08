use epaint::{
    mutex::RwLock, textures::TextureOptions, ImageData, TextureHandle, TextureId, TextureManager,
};
use std::sync::Arc;

// Taken from egui
#[derive(Clone)]
pub struct RadiantTextureManager(pub Arc<RwLock<TextureManager>>);

impl Default for RadiantTextureManager {
    fn default() -> Self {
        let mut tex_mngr = TextureManager::default();

        // Will be filled in later
        let font_id = tex_mngr.alloc(
            "egui_font_texture".into(),
            epaint::FontImage::new([0, 0]).into(),
            Default::default(),
        );
        assert_eq!(font_id, TextureId::default());

        Self(Arc::new(RwLock::new(tex_mngr)))
    }
}

impl RadiantTextureManager {
    pub fn load_texture(
        &self,
        name: impl Into<String>,
        image: impl Into<ImageData>,
        options: TextureOptions,
    ) -> TextureHandle {
        let name = name.into();
        let image = image.into();
        // let max_texture_side = 256;
        let tex_mngr = self.tex_manager();
        let tex_id = self.0.write().alloc(name, image, options);
        TextureHandle::new(tex_mngr, tex_id)
    }

    pub fn tex_manager(&self) -> Arc<RwLock<epaint::textures::TextureManager>> {
        self.0.clone()
    }
}
