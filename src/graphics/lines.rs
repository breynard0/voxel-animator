use wgpu::util::DeviceExt;

use crate::utils;

use super::{
    init,
    vertex::{self, Vertex},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub pos0: glam::Vec3,
    pub pos1: glam::Vec3,
    pub color: [f32; 4],
}

pub struct LineRenderer {
    pub lines: Vec<Line>,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_buffer_size: u32,
    pub index_buffer: wgpu::Buffer,
    pub index_buffer_size: u32,
    pub pipeline: wgpu::RenderPipeline,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub shader: wgpu::ShaderModule,
    pub depth_texture: super::texture::Texture,
    changed: bool,
}

impl LineRenderer {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        uniform_bg: &wgpu::BindGroupLayout,
    ) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Line Renderer Pipeline Layout"),
            bind_group_layouts: &[uniform_bg],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/lines.wgsl"));

        let pipeline =
            init::create_render_pipeline(device, &pipeline_layout, &shader, config, true);

        let depth_texture =
            super::depth::create_depth_texture(device, config, "line_depth_texture");

        let mut x = Self {
            lines: vec![],
            vertex_buffer: utils::empty_buffer(device),
            vertex_buffer_size: 0,
            index_buffer: utils::empty_buffer(device),
            index_buffer_size: 0,
            pipeline,
            pipeline_layout,
            shader,
            depth_texture,
            changed: false,
        };

        x.generate_buffers(&device);

        x
    }

    pub fn generate_buffers(&mut self, device: &wgpu::Device) {
        let mut vertices = vec![];
        let mut indices = vec![];

        self.vertex_buffer_size = 0;
        self.index_buffer_size = 0;

        for l in &self.lines {
            vertices.push(vertex::Vertex::new(
                l.pos0.to_array(),
                l.color,
                [0.0, 0.0, 0.0],
            ));
            vertices.push(vertex::Vertex::new(
                l.pos1.to_array(),
                l.color,
                [0.0, 0.0, 0.0],
            ));

            self.vertex_buffer_size += 2;

            indices.push(self.vertex_buffer_size - 2);
            indices.push(self.vertex_buffer_size - 1);

            self.index_buffer_size += 2;
        }

        self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn draw_line(&mut self, pos0: glam::Vec3, pos1: glam::Vec3, color: [f32; 4]) {
        self.lines.push(Line { pos0, pos1, color });
        self.changed = true;
    }
}

pub struct LineRendering {
    pub fg: LineRenderer,
    pub bg: LineRenderer,
}

impl LineRendering {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        uniform_bg: &wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            fg: LineRenderer::new(device, config, uniform_bg),
            bg: LineRenderer::new(device, config, uniform_bg),
        }
    }

    pub fn draw_line_fg(&mut self, pos0: glam::Vec3, pos1: glam::Vec3, color: [f32; 4]) {
        self.fg.draw_line(pos0, pos1, color);
    }

    pub fn draw_line_bg(&mut self, pos0: glam::Vec3, pos1: glam::Vec3, color: [f32; 4]) {
        self.bg.draw_line(pos0, pos1, color);
    }

    pub fn clear_lines_fg(&mut self) {
        self.fg.lines.clear();
        self.fg.changed = true;
    }

    pub fn clear_lines_bg(&mut self) {
        self.bg.lines.clear();
        self.bg.changed = true;
    }
}
