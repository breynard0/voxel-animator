use glam::{mat4, vec4};
use wgpu::util::DeviceExt;

use crate::utils::consts;

use super::{vertex, wgpu_object::WgpuObject};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    pub zoom: f32,
    zoom_factor: f32,
    pub offset: [f32; 2],
}

impl Default for TransformUniform {
    fn default() -> Self {
        Self {
            zoom: -5.0,
            offset: Default::default(),
            zoom_factor: 0.0,
        }
    }
}

impl TransformUniform {
    pub fn create_staging_buffer(self, device: &wgpu::Device) -> wgpu::Buffer {
        let mut uniform = self;
        uniform.zoom_factor = (1.0 + consts::ZOOM_SENS).powf(self.zoom);
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Tranform Staging Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_SRC,
        })
    }
}

pub fn clamped_rotation(wobj: &WgpuObject, clamp: f32) -> glam::Vec3 {
    wobj.cam_rotation.clamp(
        glam::vec3(f32::NEG_INFINITY, -clamp, f32::NEG_INFINITY),
        glam::vec3(f32::INFINITY, clamp, f32::INFINITY),
    )
}

pub fn trans_mat(x: f32, y: f32, z: f32) -> glam::Mat4 {
    mat4(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(x, y, z, 1.0),
    )
}

pub fn translate(vertices: &Vec<vertex::Vertex>, translation: glam::Vec3) -> Vec<vertex::Vertex> {
    let mut output = vec![];

    for vertex in vertices {
        use glam::*;

        let mut vertex = vertex.clone();
        let transmat = mat4(
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4(translation.x, translation.y, translation.z, 1.0),
        );
        let pos = [vertex.pos[0], vertex.pos[1], vertex.pos[2], 1.];
        let pos_out = transmat.mul_vec4(pos.into()).xyz().to_array();
        vertex.pos = pos_out;
        output.push(vertex);
    }

    output
}

pub fn rot_mat(x: f32, y: f32, z: f32) -> (glam::Mat4, glam::Mat4, glam::Mat4) {
    let rotmatx = mat4(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, x.cos(), x.sin(), 0.0),
        vec4(0.0, -x.sin(), x.cos(), 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    );
    let rotmaty = mat4(
        vec4(y.cos(), 0.0, -y.sin(), 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(y.sin(), 0.0, y.cos(), 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    );
    let rotmatz = mat4(
        vec4(z.cos(), z.sin(), 0.0, 0.0),
        vec4(-z.sin(), z.cos(), 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    );

    (rotmatx, rotmaty, rotmatz)
}

pub fn rotate(vertices: &Vec<vertex::Vertex>, rotation: glam::Vec3) -> Vec<vertex::Vertex> {
    let mut output = vec![];
    let x = rotation.x;
    let y = rotation.y;
    let z = rotation.z;

    let rotmat = rot_mat(x, y, z);
    let rotmat = rotmat.0 * rotmat.1 * rotmat.2;

    for vertex in vertices {
        let mut vertex = vertex.clone();

        let pos = [vertex.pos[0], vertex.pos[1], vertex.pos[2], 1.];
        let pos_out = glam::Vec4Swizzles::xyz(rotmat.mul_vec4(pos.into())).to_array();
        vertex.pos = pos_out;

        output.push(vertex);
    }

    output
}

pub fn scale(vertices: &Vec<vertex::Vertex>, scaling: glam::Vec3) -> Vec<vertex::Vertex> {
    let mut output = vec![];

    for vertex in vertices {
        use glam::*;

        let mut vertex = vertex.clone();
        let scalemat = mat4(
            vec4(scaling.x, 0.0, 0.0, 0.0),
            vec4(0.0, scaling.y, 0.0, 0.0),
            vec4(0.0, 0.0, scaling.z, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        );
        let pos = [vertex.pos[0], vertex.pos[1], vertex.pos[2], 1.];
        let pos_out = scalemat.mul_vec4(pos.into()).xyz().to_array();
        vertex.pos = pos_out;
        output.push(vertex);
    }

    output
}
