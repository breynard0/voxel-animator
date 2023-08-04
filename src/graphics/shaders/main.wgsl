struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

// const FOV = 90.0;
// fn aspectratio() -> f32 {
//     return 16.0/9.0;
// }
// const ZNEAR = 0.0;
// const ZFAR= 1.0;

// fn perspective_proj_mat() -> mat4x4<f32> {
//     var zrange = ZNEAR - ZFAR;
//     return mat4x4<f32>(
//         vec4(1.0/tan((FOV/2.0)*aspectratio()), 0.0, 0.0, 0.0),
//         vec4(0.0, 1.0/tan(FOV/2.0), 0.0, 0.0),
//         vec4(0.0, 0.0, (-ZNEAR - ZFAR) / zrange, 2.0*ZFAR*ZNEAR/zrange),
//         vec4(0.0, 0.0, 1.0, 0.0)
//     );
// }

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.color = model.color;
    // out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.xyz, 1.0);
}
