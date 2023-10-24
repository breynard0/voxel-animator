use std::f32::consts::PI;

use winit::window::Window;

use super::{cam, init, input, transform, vertex};

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
    pub transform_uniform: transform::TransformUniform,
    pub transform_buf: wgpu::Buffer,
    pub transform_staging_buf: Option<wgpu::Buffer>,
    pub uniform_bind_group: wgpu::BindGroup,
    pub msaa_buffer: wgpu::TextureView,
    pub msaa_bundle: wgpu::RenderBundle,
    pub depth_texture: super::texture::Texture,
    pub wireframe: bool,
    pub cam_rotation: glam::Vec3,
    pub cam_pos: glam::Vec3,
    pub cam_temp: cam::CamTemp,
    pub delta_time: f32,
}

impl WgpuObject {
    pub const SAMPLE_COUNT: u32 = 8;

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn update(&mut self) {
        // Camera movement
        if input::is_mouse_button_down(input::InputMouseButton::Middle) {
            let x = input::get_mouse_delta_range(self.size).0 * self.delta_time;
            let y = input::get_mouse_delta_range(self.size).1 * self.delta_time;

            // Recalculate flip factor if first frame button is held
            if !self.cam_temp.button_held_last_frame {
                // Is camera upside down?
                self.cam_temp.cam_flipped = self.cam_rotation.y < -(PI / 2.0);
            }

            let throttle = input::is_alt_down();

            match input::is_shift_down() {
                // Pan
                true => {
                    let speed = crate::utils::consts::PAN_SENS;
                    let x = x * speed;
                    let y = -y * speed;

                    let up = glam::vec3(self.cam.up.x, self.cam.up.y, self.cam.up.z);
                    let translation = self.cam.get_right() * x + up * y;
                    self.cam_pos += glam::vec3(translation.x, translation.y, translation.z);
                }
                // Orbit
                false => {
                    let speed = crate::utils::consts::ROT_SENS * match throttle {
                        true => crate::utils::consts::ROT_SENS_THROTTLE,
                        false => 1.0,
                    };

                    let x = x * speed * (!self.cam_temp.cam_flipped as u8 as f32 * 2.0 - 1.0);

                    let y = y * speed;

                    self.cam_rotation.y += y;

                    // Roll over when gets to end of range
                    if self.cam_rotation.y > PI {
                        self.cam_rotation.y = -PI;
                    }

                    if self.cam_rotation.y < -PI {
                        self.cam_rotation.y = PI;
                    }

                    self.cam_rotation.x += x;

                    if self.cam_rotation.x > PI {
                        self.cam_rotation.x = -PI;
                    }

                    if self.cam_rotation.x < -PI {
                        self.cam_rotation.x = PI;
                    }
                }
            }

            // println!("{:?}", self.cam_rotation.to_string());
            self.cam.apply_transforms(&self.cam_rotation, &self.cam_pos);
            self.cam_staging_buf = Some(self.cam.create_staging_buffer(&self.device));
            self.cam_temp.button_held_last_frame = true;
        } else {
            self.cam_temp.button_held_last_frame = false;
        }

        let prezoom = self.transform_uniform.zoom;
        self.transform_uniform.zoom += input::get_scroll_delta();
        if self.transform_uniform.zoom != prezoom {
            self.transform_staging_buf =
                Some(self.transform_uniform.create_staging_buffer(&self.device));
        }

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
