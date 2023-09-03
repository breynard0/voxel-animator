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
    // let new_pos: vec3<f32> = vec3<f32>(
    //         model.position.x * (transform.zoom * (model.position.x / abs(model.position.x))),
    //         model.position.y * (transform.zoom * (model.position.y / abs(model.position.y))),
    //         model.position.z * (transform.zoom * (model.position.z / abs(model.position.z)))
    //     );

    // let pos_x = vec4((model.position * transform.zoom_factor), 0.0) * (transform.rot_mat_x);
    // let pos_y = vec4((model.position * transform.zoom_factor), 0.0) * (transform.rot_mat_y);
    // let pos_z = vec4((model.position * transform.zoom_factor), 0.0) * (transform.rot_mat_z);
    
    // out.clip_position = camera.view_proj * vec4<f32>(vec3(pos_x.x, pos_y.y, (length(pos_x)+length(pos_y)) / 2.0), 1.0);
    out.clip_position = camera.view_proj * vec4<f32>(model.position * transform.zoom_factor, 1.0);
    out.color = model.color;
    return out;
}

// NOTE: Colour should be that of the vertex with smaller position
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.xyz, 1.0);
}
