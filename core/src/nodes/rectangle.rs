use crate::{RenderComponent, SelectionComponent, RadiantIdentifiable, RadiantSelectable};
use crate::RadiantMessageHandler;
use crate::SelectionMessage;
use crate::TransformMessage;
use crate::{RadiantRenderable, RadiantVertex, TransformComponent};
use serde::{Deserialize, Serialize};
use crate::RadiantScene;

const VERTICES: &[RadiantVertex] = &[
    RadiantVertex {
        position: [0.5, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    RadiantVertex {
        position: [-0.5, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    RadiantVertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    RadiantVertex {
        position: [0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RadiantRectangleMessage {
    Transform(TransformMessage),
    Selection(SelectionMessage),
}

#[derive(Serialize, Deserialize)]
pub struct RadiantRectangleNode {
    pub id: u64,
    pub transform: TransformComponent,
    pub selection: SelectionComponent,
    #[serde(skip)]
    pub renderer: Option<RenderComponent>,
    #[serde(skip)]
    pub offscreen_renderer: Option<RenderComponent>,
}

impl RadiantRectangleNode {
    pub fn new(
        id: u64,
        position: [f32; 2],
    ) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_xy(&position);

        let selection = SelectionComponent::new();

        Self {
            id,
            transform,
            selection,
            renderer: None,
            offscreen_renderer: None
        }
    }
}

impl RadiantIdentifiable for RadiantRectangleNode {
    fn get_id(&self) -> u64 {
        return self.id;
    }
}

impl RadiantSelectable for RadiantRectangleNode {
    fn set_selected(&mut self, selected: bool) {
        self.selection.set_selected(selected);
        self.renderer.as_mut().map_or((), |r| 
            r.set_selection_color([1.0, 0.0, 0.0, if selected { 1.0 } else { 0.0 }]));
    }
}

impl RadiantRenderable for RadiantRectangleNode {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene) {
        let mut renderer = RenderComponent::new(&scene.device, scene.config.format, &VERTICES, &INDICES);
        renderer.set_position(&self.transform.get_xy());

        let mut offscreen_renderer =
            RenderComponent::new(&scene.device, wgpu::TextureFormat::Rgba8Unorm, &VERTICES, &INDICES);
        offscreen_renderer.set_position(&self.transform.get_xy());
        offscreen_renderer.set_selection_color([
            ((self.id + 1 >> 0) & 0xFF) as f32 / 0xFF as f32,
            ((self.id + 1 >> 8) & 0xFF) as f32 / 0xFF as f32,
            ((self.id + 1 >> 16) & 0xFF) as f32 / 0xFF as f32,
            1.0,
        ]);

        self.renderer = Some(renderer);
        self.offscreen_renderer = Some(offscreen_renderer);
    }

    fn detach(&mut self) {
        self.renderer = None;
        self.offscreen_renderer = None;
    }

    fn update(&mut self, queue: &mut wgpu::Queue) {
        self.renderer.as_mut().map_or((), |r| r.update(queue));
        self.offscreen_renderer.as_mut().map_or((), |r| r.update(queue));
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool) {
        log::debug!("Rendering rectangle");
        if offscreen {
            self.offscreen_renderer.as_ref().map_or((), |r| r.render(render_pass));
        } else {
            self.renderer.as_ref().map_or((), |r| r.render(render_pass));
        }
    }
}

impl RadiantMessageHandler<RadiantRectangleMessage> for RadiantRectangleNode {
    fn handle_message(&mut self, message: RadiantRectangleMessage) {
        match message {
            RadiantRectangleMessage::Transform(message) => {
                self.transform.handle_message(message);
                // self.renderer.set_position(&self.transform.get_xy());
                // self.offscreen_renderer.set_position(&self.transform.get_xy());
            }
            RadiantRectangleMessage::Selection(message) => {
                self.selection.handle_message(message);
                // self.renderer
                //     .set_selection_color([1.0, 0.0, 0.0, if self.selection.is_selected() { 1.0 } else { 0.0 }]);
            }
        }
    }
}