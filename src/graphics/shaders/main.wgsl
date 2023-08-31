struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct TransformUniform {
    rot_mat: mat4x4<f32>,
    zoom: f32,
    offset: vec2<f32>
}
@group(0) @binding(1)
var<uniform> transform: TransformUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position + (1. - 1. / clamp(transform.zoom, 0.01, 100000000.0)), 1.0) * transform.rot_mat;
    out.color = model.color;
    return out;
}

// NOTE: Colour should be that of the vertex with smaller position
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.xyz, 1.0);
}
