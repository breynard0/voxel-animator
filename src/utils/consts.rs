#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub const VBO_ATTRIBS: [wgpu::VertexAttribute; 3] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4, 2 => Float32x3];

pub const ZOOM_SENS: f32 = 0.2;

pub const ROT_SENS: f32 = 500.0;

pub const ROT_CLAMP: f32 = (std::f32::consts::PI / 2.0) * 0.999;

pub const PAN_SENS: f32 = 100.0;
