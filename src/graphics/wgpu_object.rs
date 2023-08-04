use winit::window::Window;

use super::{
    cam,
    vertex::{self, Vertex},
};

pub struct WgpuObject {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_buffer_size: u32,
    pub index_buffer: wgpu::Buffer,
    pub index_buffer_size: u32,
    pub cam: cam::Camera,
    pub cam_uniform: cam::CameraUniform,
    pub cam_buf: wgpu::Buffer,
    pub cam_bind_group: wgpu::BindGroup,
    pub msaa_buffer: wgpu::TextureView,
    pub rotation: glam::Vec3,
    // pub rpass_bundle: wgpu::RenderBundle,
}

impl WgpuObject {
    pub const SAMPLE_COUNT: u32 = 1;

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn update(&mut self) {
        self.rotation.x += 0.001;
        self.rotation.y += 1.0;
        self.rotation.z += 1.0;
        if self.rotation.x >= 360.0 {
            self.rotation.x = 0.0;
        }
        self.vertex_buffer = vertex::new_vbo(&self.device, self.rotation)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
