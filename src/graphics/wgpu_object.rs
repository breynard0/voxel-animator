use winit::window::Window;

use super::{cam, init, input, vertex};

pub struct WgpuObject {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,
    pub pipeline: wgpu::RenderPipeline,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub shader: wgpu::ShaderModule,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_buffer_size: u32,
    pub index_buffer: wgpu::Buffer,
    pub index_buffer_size: u32,
    pub cam: cam::Camera,
    pub cam_uniform: cam::CameraUniform,
    pub cam_buf: wgpu::Buffer,
    pub cam_staging_buf: Option<wgpu::Buffer>,
    pub cam_bind_group: wgpu::BindGroup,
    pub msaa_buffer: wgpu::TextureView,
    pub msaa_bundle: wgpu::RenderBundle,
    pub depth_texture: super::texture::Texture,
    pub wireframe: bool,
    pub rotation: glam::Vec3,
    pub delta_time: f32,
}

impl WgpuObject {
    pub const SAMPLE_COUNT: u32 = 8;

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn update(&mut self) {
        if input::is_mouse_button_down(input::InputMouseButton::Middle) {
            let speed = 1000.0;
            self.rotation.x += input::get_mouse_delta_range(self.size).1 * self.delta_time * speed;
            self.rotation.z += input::get_mouse_delta_range(self.size).0 * self.delta_time * speed;
        }

        self.vertex_buffer = super::vertex::new_vbo(&self.device, self.rotation);
        super::msaa::rebuild_msaa(self);
        if input::is_key_pressed(winit::event::VirtualKeyCode::F1) {
            self.wireframe = !self.wireframe;
            let vib = vertex::create_buffers(&self.device, self.wireframe);
            self.index_buffer = vib.idxbuf;
            self.index_buffer_size = vib.idx_size;
            self.pipeline = init::create_render_pipeline(
                &self.device,
                &self.pipeline_layout,
                &self.shader,
                &self.config,
                self.wireframe,
            );
        }

        input::input_update();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.depth_texture =
                super::depth::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.surface.configure(&self.device, &self.config);
            super::msaa::rebuild_msaa(self);
            self.cam.aspect = new_size.width as f32 / new_size.height as f32;
            self.cam_staging_buf = Some(self.cam.create_staging_buffer(&self.device));
        }
    }
}
