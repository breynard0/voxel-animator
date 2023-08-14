use bytemuck::*;
use wgpu::{util::DeviceExt, Buffer};

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
    1, 2, 3,
    4, 5, 7,
    5, 6, 7,
    1, 5, 2,
    5, 6, 2,
    4, 7, 3,
    0, 4, 3,
    0, 4, 1,
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

pub struct BufferOutput<'a> {
    pub vbo: Buffer,
    pub vbodesc: wgpu::VertexBufferLayout<'a>,
    pub vbo_size: u32,
    pub idxbuf: Buffer,
    pub idx_size: u32,
}

pub fn new_vbo(device: &wgpu::Device, angles: glam::Vec3) -> wgpu::Buffer {
    let mut vertices = vec![];

    for vertex in VERTICES {
        use glam::*;

        let mut vertex = vertex.clone();
        let x = angles.x;
        let rotmat = mat4(
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, x.cos(), x.sin(), 0.0),
            vec4(0.0, -x.sin(), x.cos(), 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        );

        let pos = [vertex.pos[0], vertex.pos[1], vertex.pos[2], 1.];
        let pos_out = rotmat.mul_vec4(pos.into()).xyz().to_array();
        vertex.pos = pos_out;

        vertices.push(vertex);
    }

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("VertexBuffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

pub fn create_buffers(device: &wgpu::Device) -> BufferOutput<'static> {
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &ATTRIBS,
    };

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("VertexBuffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });

    // let mut indices = vec![];

    // for i in INDICES.chunks(3) {
    //     indices.push(i[0]);
    //     indices.push(i[1]);
    //     indices.push(i[1]);
    //     indices.push(i[2]);
    //     indices.push(i[2]);
    //     indices.push(i[0]);
    // }

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });

    BufferOutput {
        vbo: vertex_buffer,
        vbodesc: vertex_buffer_layout,
        vbo_size: VERTICES.len() as u32,
        idxbuf: index_buffer,
        idx_size: INDICES.len() as u32,
    }
}
