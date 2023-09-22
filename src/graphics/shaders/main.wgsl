struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct TransformUniform {
    // rot_mat_x: mat4x4<f32>,
    // rot_mat_y: mat4x4<f32>,
    // rot_mat_z: mat4x4<f32>,
    zoom: f32,
    zoom_factor: f32,
    offset: vec2<f32>,
}
@group(0) @binding(1)
var<uniform> transform: TransformUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) normal: vec3<f32>
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
    
    out.clip_position = camera.view_proj * vec4<f32>(model.position * transform.zoom_factor, 1.0);
    out.color = vec4((model.normal+1.0)/2.0, 1.0);
    return out;
}

// NOTE: Colour should be that of the vertex with smaller position
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.xyz, 1.0);
}
