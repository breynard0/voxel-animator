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

pub const ROT_SENS: f32 = 1000.0;

pub const ROT_SENS_THROTTLE: f32 = 0.2;

pub const PAN_SENS: f32 = 100.0;

pub const PAN_SENS_THROTTLE: f32 = 0.2;

#[rustfmt::skip]
pub const INDICES_TOP: &[u32] = &[
    0, 1, 3,
    3, 1, 2
];

#[rustfmt::skip]
pub const INDICES_BOTTOM: &[u32] = &[
    4, 5, 7,
    5, 6, 7
];

#[rustfmt::skip]
pub const INDICES_FRONT: &[u32] = &[
    1, 5, 2,
    5, 6, 2
];
#[rustfmt::skip]
pub const INDICES_BACK: &[u32] = &[
    4, 7, 3,
    0, 4, 3
];
#[rustfmt::skip]
pub const INDICES_LEFT: &[u32] = &[
    0, 1, 4,
    4, 5, 1
];
#[rustfmt::skip]
pub const INDICES_RIGHT: &[u32] = &[
    7, 6, 2,
    7, 2, 3
];
