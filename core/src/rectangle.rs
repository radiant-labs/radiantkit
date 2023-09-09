use super::{RadiantNode, RadiantNodeRenderable, TransformComponent};
use std::sync::Arc;
use wgpu::util::DeviceExt;

// lib.rs
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ModelUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    model_view: [[f32; 4]; 4],
}

impl ModelUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            model_view: cgmath::Matrix4::identity().into(),
        }
    }

    fn translate(&mut self, position: &[f32; 2]) {
        self.model_view =
            cgmath::Matrix4::from_translation(cgmath::Vector3::new(position[0], position[1], 0.0))
                .into();
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct SelectionUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    selected: [f32; 4],
}

impl SelectionUniform {
    fn new() -> Self {
        Self {
            selected: [1.0, 0.0, 0.0, 1.0],
        }
    }
}

// lib.rs
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.5, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.5, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

pub struct RadiantRectangleNode {
    pub transform: TransformComponent,
    dirty: bool,
    render_pipeline: Arc<wgpu::RenderPipeline>,
    offscreen_render_pipeline: Arc<wgpu::RenderPipeline>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    model_uniform: ModelUniform,
    model_buffer: wgpu::Buffer,
    model_bind_group: wgpu::BindGroup,
    selection_uniform: SelectionUniform,
    selection_buffer: wgpu::Buffer,
    selection_bind_group: wgpu::BindGroup,
    selection_uniform_2: SelectionUniform,
    selection_buffer_2: wgpu::Buffer,
    selection_bind_group_2: wgpu::BindGroup,
}

impl RadiantRectangleNode {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        position: [f32; 2],
    ) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_xy(&position);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        let mut model_uniform = ModelUniform::new();
        model_uniform.translate(&position);
        let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Buffer"),
            contents: bytemuck::cast_slice(&[model_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("model_bind_group_layout"),
            });
        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &model_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model_buffer.as_entire_binding(),
            }],
            label: Some("model_bind_group"),
        });

        let mut selection_uniform = SelectionUniform::new();
        let selection_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Selection Buffer"),
            contents: bytemuck::cast_slice(&[selection_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let selection_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("selection_bind_group_layout"),
            });
        let selection_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &selection_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: selection_buffer.as_entire_binding(),
            }],
            label: Some("selection_bind_group"),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&model_bind_group_layout, &selection_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",     // 1.
                    buffers: &[Vertex::desc()], // 2.
                },
                fragment: Some(wgpu::FragmentState {
                    // 3.
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        // 4.
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None, // 1.
                multisample: wgpu::MultisampleState {
                    count: 1,                         // 2.
                    mask: !0,                         // 3.
                    alpha_to_coverage_enabled: false, // 4.
                },
                multiview: None, // 5.
            })
            .into();

        let mut selection_uniform_2 = SelectionUniform::new();
        let selection_buffer_2 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Selection Buffer"),
            contents: bytemuck::cast_slice(&[selection_uniform_2]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let selection_bind_group_layout_2 =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("selection_bind_group_layout"),
            });
        let selection_bind_group_2 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &selection_bind_group_layout_2,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: selection_buffer_2.as_entire_binding(),
            }],
            label: Some("selection_bind_group"),
        });
        let selection_shader =
            device.create_shader_module(wgpu::include_wgsl!("shader_selection.wgsl"));
        let selection_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&model_bind_group_layout, &selection_bind_group_layout_2],
                push_constant_ranges: &[],
            });
        let offscreen_render_pipeline = device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&selection_render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &selection_shader,
                    entry_point: "vs_main",     // 1.
                    buffers: &[Vertex::desc()], // 2.
                },
                fragment: Some(wgpu::FragmentState {
                    // 3.
                    module: &selection_shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        // 4.
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None, // 1.
                multisample: wgpu::MultisampleState {
                    count: 1,                         // 2.
                    mask: !0,                         // 3.
                    alpha_to_coverage_enabled: false, // 4.
                },
                multiview: None, // 5.
            })
            .into();

        Self {
            transform,
            dirty: false,
            model_uniform,
            model_buffer,
            render_pipeline,
            offscreen_render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            model_bind_group,
            selection_uniform,
            selection_buffer,
            selection_bind_group,
            selection_uniform_2,
            selection_buffer_2,
            selection_bind_group_2,
        }
    }
}

impl RadiantNode for RadiantRectangleNode {
    fn set_selected(&mut self, selected: bool) {
        self.selection_uniform.selected[3] = if selected { 1.0 } else { 0.0 };
        self.dirty = true;
    }

    fn set_id(&mut self, id: u64) {
        self.selection_uniform_2.selected[0] = ((id + 1 >> 0) & 0xFF) as f32 / 0xFF as f32;
        self.selection_uniform_2.selected[1] = ((id + 1 >> 8) & 0xFF) as f32 / 0xFF as f32;
        self.selection_uniform_2.selected[2] = ((id + 1 >> 16) & 0xFF) as f32 / 0xFF as f32;
        self.dirty = true;

        // log::info!("id: {}", id);
        // log::info!("uniforms: {} {} {}", self.selection_uniform.selected[0], self.selection_uniform.selected[1], self.selection_uniform.selected[2]);

        // if id == 1 {
        //     self.set_selected(true);
        // }
    }
}

impl RadiantNodeRenderable for RadiantRectangleNode {
    fn update(&mut self, queue: &mut wgpu::Queue) {
        if self.dirty {
            queue.write_buffer(
                &self.model_buffer,
                0,
                bytemuck::cast_slice(&[self.model_uniform]),
            );
            queue.write_buffer(
                &self.selection_buffer,
                0,
                bytemuck::cast_slice(&[self.selection_uniform]),
            );
            queue.write_buffer(
                &self.selection_buffer_2,
                0,
                bytemuck::cast_slice(&[self.selection_uniform_2]),
            );
            self.dirty = false;
        }
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool) {
        log::debug!("Rendering rectangle");

        if offscreen {
            render_pass.set_pipeline(&self.offscreen_render_pipeline);
            render_pass.set_bind_group(1, &self.selection_bind_group_2, &[]);
        } else {
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(1, &self.selection_bind_group, &[]);
        }

        render_pass.set_bind_group(0, &self.model_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
