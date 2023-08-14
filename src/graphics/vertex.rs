use bytemuck::*;
use wgpu::{util::DeviceExt, Buffer};

use super::transform;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 4],
}

const ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4];

const VERTICES: &[Vertex] = &[
    Vertex {
        pos: [-0.5, 0.5, -0.5],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        pos: [-0.5, 0.5, 0.5],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        pos: [0.5, 0.5, 0.5],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        pos: [0.5, 0.5, -0.5],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        pos: [-0.5, -0.5, -0.5],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        pos: [-0.5, -0.5, 0.5],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        pos: [0.5, -0.5, 0.5],
        color: [0.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        pos: [0.5, -0.5, -0.5],
        color: [1.0, 1.0, 1.0, 1.0],
    },
];

#[rustfmt::skip]
const INDICES: &[u32] = &[
    0, 1, 3,
    3, 1, 2,
    4, 5, 7,
    5, 6, 7,
    1, 5, 2,
    5, 6, 2,
    4, 7, 3,
    0, 4, 3,
    0, 1, 4,
    4, 5, 1,
    7, 6, 2,
    7, 2, 3
];

// const VERTICES: &[Vertex] = &[
//     Vertex { pos: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5, 1.0] },
//     Vertex { pos: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5, 1.0] },
//     Vertex { pos: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5, 1.0] },
//     Vertex { pos: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5, 1.0] },
//     Vertex { pos: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5, 1.0] },
// ];

// const INDICES: &[u32] = &[
//     0, 1, 4,
//     1, 2, 4,
//     2, 3, 4,
// ];

pub struct BufferOutput {
    pub vbo: Buffer,
    pub vbo_size: u32,
    pub idxbuf: Buffer,
    pub idx_size: u32,
}

pub fn new_vbo(device: &wgpu::Device, angles: glam::Vec3) -> wgpu::Buffer {
    let vertices = transform::rotate(&VERTICES.to_vec(), angles);

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("VertexBuffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

pub fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &ATTRIBS,
    };
    vertex_buffer_layout
}

pub fn create_buffers(device: &wgpu::Device, wireframe: bool) -> BufferOutput {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("VertexBuffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let indices = match wireframe {
        true => {
            let mut indices = vec![];

            for i in INDICES.chunks(3) {
                indices.push(i[0]);
                indices.push(i[1]);
                indices.push(i[1]);
                indices.push(i[2]);
                indices.push(i[2]);
                indices.push(i[0]);
            }
            indices
        }
        false => INDICES.to_vec(),
    };

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    BufferOutput {
        vbo: vertex_buffer,
        vbo_size: VERTICES.len() as u32,
        idxbuf: index_buffer,
        idx_size: indices.len() as u32,
    }
}
