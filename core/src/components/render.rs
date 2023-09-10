use crate::RadiantVertex;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct VertexUniform {
    model_view: [[f32; 4]; 4],
}

impl VertexUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            model_view: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn translate(&mut self, position: &[f32; 2]) {
        self.model_view =
            cgmath::Matrix4::from_translation(cgmath::Vector3::new(position[0], position[1], 0.0))
                .into();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct FragmentUniform {
    pub selected: [f32; 4],
}

impl FragmentUniform {
    fn new() -> Self {
        Self {
            selected: [1.0, 0.0, 0.0, 1.0],
        }
    }
}

pub struct RenderComponent {
    vertex_uniform: VertexUniform,
    fragment_uniform: FragmentUniform,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    vertex_uniform_buffer: wgpu::Buffer,
    vertex_uniform_bind_group: wgpu::BindGroup,
    fragment_uniform_buffer: wgpu::Buffer,
    fragment_uniform_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    dirty: bool,
}

impl RenderComponent {
    pub fn new(
        device: &wgpu::Device,
        target_texture_format: wgpu::TextureFormat,
        vertices: &[RadiantVertex],
        indices: &[u16],
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = indices.len() as u32;

        let vertex_uniform = VertexUniform::new();

        let vertex_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Uniform Buffer"),
            contents: bytemuck::cast_slice(&[vertex_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let vertex_bind_group_layout =
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
                label: Some("vertex_bind_group_layout"),
            });

        let vertex_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &vertex_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: vertex_uniform_buffer.as_entire_binding(),
            }],
            label: Some("vertex_bind_group"),
        });

        let fragment_uniform = FragmentUniform::new();

        let fragment_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Fragment Uniform Buffer"),
                contents: bytemuck::cast_slice(&[fragment_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let fragment_bind_group_layout =
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
                label: Some("fragment_bind_group_layout"),
            });

        let fragment_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &fragment_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: fragment_uniform_buffer.as_entire_binding(),
            }],
            label: Some("fragment_bind_group"),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&vertex_bind_group_layout, &fragment_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",            // 1.
                    buffers: &[RadiantVertex::desc()], // 2.
                },
                fragment: Some(wgpu::FragmentState {
                    // 3.
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        // 4.
                        format: target_texture_format,
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
            vertex_uniform,
            fragment_uniform,
            vertex_buffer,
            index_buffer,
            num_indices,
            vertex_uniform_buffer,
            vertex_uniform_bind_group,
            fragment_uniform_buffer,
            fragment_uniform_bind_group,
            render_pipeline,
            dirty: false,
        }
    }

    pub fn set_position(&mut self, position: &[f32; 2]) {
        self.vertex_uniform.translate(position);
        self.dirty = true;
    }

    pub fn set_selection_color(&mut self, color: [f32; 4]) {
        self.fragment_uniform.selected = color;
        self.dirty = true;
    }

    pub fn update(&mut self, queue: &mut wgpu::Queue) {
        if self.dirty {
            queue.write_buffer(
                &self.vertex_uniform_buffer,
                0,
                bytemuck::cast_slice(&[self.vertex_uniform]),
            );
            queue.write_buffer(
                &self.fragment_uniform_buffer,
                0,
                bytemuck::cast_slice(&[self.fragment_uniform]),
            );
            self.dirty = false;
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.vertex_uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &self.fragment_uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
