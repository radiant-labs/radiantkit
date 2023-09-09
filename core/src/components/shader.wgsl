// Vertex shader
struct ModelUniform {
    model_view: mat4x4<f32>,
};
@group(0) @binding(0) // 1.
var<uniform> model_uniform: ModelUniform;

struct SelectionUniform {
    selected: vec4<f32>,
}
@group(1) @binding(0) // 1.
var<uniform> selection_uniform: SelectionUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = model_uniform.model_view * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if selection_uniform.selected[3] > 0.5 {
        return vec4<f32>(selection_uniform.selected.xyz, 1.0);
    } else {
        return vec4<f32>(in.color, 1.0);
    }
}
