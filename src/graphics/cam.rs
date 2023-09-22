use cgmath::InnerSpace;
use wgpu::util::DeviceExt;

use crate::utils::consts::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn create_staging_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let mut camera_uniform = super::cam::CameraUniform::new();
        camera_uniform.update_view_proj(self);

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Staging Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_SRC,
        })
    }

    pub fn apply_transforms(&mut self, rot: &glam::Vec3, pos: &glam::Vec3) {
        let rot = rot;

        let rotation = cgmath::point3(
            (-rot.x).sin() * (rot.y).cos(),
            (rot.y).sin(),
            (-rot.x).cos() * (rot.y).cos(),
        );

        self.eye = rotation + cgmath::vec3(pos.x, pos.y, pos.z);
        self.target = cgmath::point3(pos.x, pos.y, pos.z);

        self.rebuild_up();
    }

    pub fn rebuild_up(&mut self) {
        let eye = glam::vec3(self.eye.x, self.eye.y, self.eye.z);
        let target = glam::vec3(self.target.x, self.target.y, self.target.z);
        let forward = (target - eye).normalize();
        let right = forward.cross(glam::vec3(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(forward).normalize();
        self.up = cgmath::vec3(up.x, up.y, up.z);
    }

    pub fn get_right(&self) -> cgmath::Vector3<f32> {
        self.up
            .cross((self.eye - self.target).normalize())
            .normalize()
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
