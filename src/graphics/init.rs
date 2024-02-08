use wgpu::{util::DeviceExt, Backends, FragmentState, Limits, TextureFormat, VertexState};

use super::{
    cam, lines, msaa, transform,
    vertex::{self},
    wgpu_object::WgpuObject,
};

use crate::{
    models::{model, regen},
    utils::{self, consts::*, log},
};

pub async fn gfx_init(window: &winit::window::Window) -> WgpuObject {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::all(),
        dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        #[cfg(debug_assertions)]
        flags: wgpu::InstanceFlags::DEBUG | wgpu::InstanceFlags::VALIDATION,
        #[cfg(not(debug_assertions))]
        flags: wgpu::InstanceFlags::empty(),
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });

    let surface = instance
        .create_surface(window)
        .expect("Failed to create surface");

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Unable to create rendering adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("render_device"),
                required_features: adapter.features(),
                required_limits: Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .expect("Unable to request rendering device and queue");

    let size = window.inner_size();
    let wireframe = false;

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb() && f == &TextureFormat::Rgba8UnormSrgb)
        .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![surface_format],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    let depth_texture = super::depth::create_depth_texture(&device, &config, "depth_texture");

    let shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/main.wgsl"));

    let vertex_index_buffer =
        vertex::create_buffers(regen::gen_vert_idx(&model::get_model()), &device, wireframe);

    let rotation = glam::Vec3::ZERO;

    let mut camera = cam::Camera {
        eye: (-2.0, 1.0, 2.0).into(),
        target: (0.0, 0.0, 0.0).into(),
        up: cgmath::Vector3::unit_y(),
        aspect: config.width as f32 / config.height as f32,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
    };

    camera.eye = cgmath::point3(
        -(-rotation.x).sin() * (rotation.y).cos(),
        -(rotation.y).sin(),
        (-rotation.x).cos() * (rotation.y).cos(),
    );

    let mut camera_uniform = cam::CameraUniform::new();
    camera_uniform.update_view_proj(&camera);

    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let transform_uniform = transform::TransformUniform::default();
    let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("transform Buffer"),
        contents: &utils::uniform_buffer_to_bytes(transform_uniform),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let transform_staging_buf = transform_uniform.create_staging_buffer(&device);

    let uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("UniformBindGroupLayout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("UniformBindGroup"),
        layout: &uniform_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: transform_buffer.as_entire_binding(),
            },
        ],
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("RenderPipelineLayout"),
        bind_group_layouts: &[&uniform_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = create_render_pipeline(
        &device,
        &render_pipeline_layout,
        &shader,
        &config,
        wireframe,
    );

    let msaa_buffer =
        msaa::create_multisampled_framebuffer(&device, &config, WgpuObject::SAMPLE_COUNT);

    let msaa_bundle = msaa::create_bundle(
        &device,
        &config,
        &render_pipeline,
        &vertex_index_buffer.vbo,
        &vertex_index_buffer.idxbuf,
        vertex_index_buffer.idx_size,
    );

    let line_rendering = lines::LineRendering::new(&device, &config, &uniform_bind_group_layout);

    let mut out = WgpuObject {
        surface,
        device,
        queue,
        config,
        size,
        window: window,
        pipeline: render_pipeline,
        pipeline_layout: render_pipeline_layout,
        shader,
        vertex_buffer: vertex_index_buffer.vbo,
        vertex_buffer_size: vertex_index_buffer.vbo_size,
        index_buffer: vertex_index_buffer.idxbuf,
        index_buffer_size: vertex_index_buffer.idx_size,
        cam: camera,
        cam_buf: camera_buffer,
        cam_staging_buf: None,
        cam_uniform: camera_uniform,
        transform_uniform,
        transform_buf: transform_buffer,
        transform_staging_buf: Some(transform_staging_buf),
        restage_transform: false,
        uniform_bind_group,
        msaa_buffer,
        msaa_bundle,
        depth_texture,
        wireframe,
        delta_time: 0.0,
        cam_rotation: rotation,
        cam_pos: glam::Vec3::ZERO,
        cam_temp: Default::default(),
        line_rendering,
    };

    out.update();

    out
}

pub fn create_render_pipeline<'a>(
    device: &wgpu::Device,
    render_pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    config: &wgpu::SurfaceConfiguration,
    wireframe: bool,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("RenderPipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[vertex::vertex_buffer_layout()],
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(config.view_formats[0].into())],
        }),
        primitive: wgpu::PrimitiveState {
            topology: match wireframe {
                true => wgpu::PrimitiveTopology::LineList,
                false => wgpu::PrimitiveTopology::TriangleList,
            },
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            // cull_mode: Some(wgpu::Face::Back),
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: WgpuObject::SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
