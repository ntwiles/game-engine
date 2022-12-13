// Vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) matrix_0: vec4<f32>,
    @location(3) matrix_1: vec4<f32>,
    @location(4) matrix_2: vec4<f32>,
    @location(5) matrix_3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let transform = mat4x4<f32>(
        model.matrix_0,
        model.matrix_1,
        model.matrix_2,
        model.matrix_3,
    );

    var out: VertexOutput;
    out.clip_position = transform * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}