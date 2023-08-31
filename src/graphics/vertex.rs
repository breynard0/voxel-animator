use bytemuck::*;
use wgpu::{util::DeviceExt, Buffer};

use crate::models::model;

use crate::utils::consts::*;

use super::transform;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex {
    pub fn new(pos: [f32; 3], color: [f32; 4]) -> Self {
        Self { pos, color }
    }
}

pub struct BufferOutput {
    pub vbo: Buffer,
    pub vbo_size: u32,
    pub idxbuf: Buffer,
    pub idx_size: u32,
}

pub fn new_vbo(device: &wgpu::Device, angles: glam::Vec3) -> wgpu::Buffer {
    let model = model::gen_vert_idx(&model::get_model());

    let vertices = transform::rotate(&model.0.to_vec(), angles);
    // let vertices = model.0;

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
        attributes: &VBO_ATTRIBS,
    };
    vertex_buffer_layout
}

pub fn create_buffers(device: &wgpu::Device, wireframe: bool) -> BufferOutput {
    let model = model::gen_vert_idx(&model::get_model());

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("VertexBuffer"),
        contents: bytemuck::cast_slice(&model.0),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let indices = match wireframe {
        true => {
            let mut indices = vec![];

            for i in model.1.chunks(3) {
                indices.push(i[0]);
                indices.push(i[1]);
                indices.push(i[1]);
                indices.push(i[2]);
                indices.push(i[2]);
                indices.push(i[0]);
            }
            indices
        }
        false => model.1.to_vec(),
    };

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    // indices.chunks(3).for_each(|x| println!("{:?}", x));

    BufferOutput {
        vbo: vertex_buffer,
        vbo_size: model.0.len() as u32,
        idxbuf: index_buffer,
        idx_size: indices.len() as u32,
    }
}
