use std::f32::consts::PI;

use cgmath::Matrix;
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
        let view = self.get_view_matrix();

        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn get_view_matrix(&self) -> cgmath::Matrix4<f32> {
        let eye = glam::vec3(self.eye.x, self.eye.y, self.eye.z);

        let r = self.get_right();
        let u = glam::vec3(self.up.x, self.up.y, self.up.z);
        let f = self.get_forward();

        let tx = eye.dot(r);
        let ty = eye.dot(u);
        let tz = eye.dot(f);

        cgmath::Matrix4 {
            x: cgmath::vec4(r.x, r.y, r.z, tx),
            y: cgmath::vec4(u.x, u.y, u.z, ty),
            z: cgmath::vec4(f.x, f.y, f.z, tz),
            w: cgmath::vec4(0.0, 0.0, 0.0, 1.0),
        }
        .transpose()
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

    pub fn apply_transforms(&mut self, rot: &glam::Vec3) {
        let rot = rot;

        let rotation = cgmath::point3(
            (-rot.x).sin() * (rot.y).cos(),
            -(rot.y).sin(),
            (-rot.x).cos() * (rot.y).cos(),
        );

        self.eye = rotation;

        self.rebuild_up(rot.y);
    }

    pub fn rebuild_up(&mut self, yrot: f32) {
        let forward = self.get_forward();
        let temp_up = match yrot.abs() > (PI / 2.0) {
            true => glam::vec3(0.0, -1.0, 0.0),
            false => glam::vec3(0.0, 1.0, 0.0),
        };
        let right = forward.cross(temp_up).normalize();
        let up = right.cross(forward).normalize();
        self.up = cgmath::vec3(up.x, up.y, up.z);
    }

    pub fn get_forward(&self) -> glam::Vec3 {
        (glam::vec3(self.target.x, self.target.y, self.target.z))
            - (glam::vec3(self.eye.x, self.eye.y, self.eye.z).normalize())
    }

    pub fn get_right(&self) -> glam::Vec3 {
        let up = glam::vec3(self.up.x, self.up.y, self.up.z);
        up.cross(self.get_forward()).normalize()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CamTemp {
    pub button_held_last_frame: bool,
    pub cam_flipped: bool,
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
