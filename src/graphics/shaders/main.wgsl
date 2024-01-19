struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct TransformUniform {
    zoom: f32,
    zoom_factor: f32,
    pan: vec3<f32>,
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
    @location(0) @interpolate(perspective) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    // out.clip_position = camera.view_proj * vec4<f32>(model.position * transform.zoom_factor, 1.0);
    out.clip_position = camera.view_proj * vec4<f32>(model.position * transform.zoom_factor + transform.pan, 1.0);
    let sun_dir = normalize(vec3<f32>(1.0, -1.0, 1.0));
    var factor = dot(normalize(model.normal), -sun_dir);
    
    out.color = model.color * factor;
    return out;
}

// NOTE: Colour should be that of the vertex with smaller position
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.xyz, 1.0);
}