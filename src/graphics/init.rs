use wgpu::{
    util::DeviceExt, Backends, BindGroupDescriptor, Features, FragmentState, Limits, TextureFormat,
    VertexState,
};

use super::{cam, msaa, vertex, wgpu_object::WgpuObject};

pub async fn gfx_init(window: winit::window::Window) -> WgpuObject {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::all(),
        dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
    });

    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Failed to create surface")
    };

    let adapter = instance
        .enumerate_adapters(wgpu::Backends::all())
        .find(|adapter| adapter.is_surface_supported(&surface))
        .expect("No graphics adapter found");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("device"),
                features: Features::all_webgpu_mask(),
                limits: Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .expect("Unable to request device and queue");

    let size = window.inner_size();

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
    };

    surface.configure(&device, &config);

    let shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/main.wgsl"));

    let vertex_index_buffer = vertex::create_buffers(&device);

    let camera = cam::Camera {
        eye: (0.0, 1.0, 2.0).into(),
        target: (0.0, 0.0, 0.0).into(),
        up: cgmath::Vector3::unit_y(),
        aspect: config.width as f32 / config.height as f32,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
    };

    let mut camera_uniform = cam::CameraUniform::new();
    camera_uniform.update_view_proj(&camera);

    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("CamBindGroup"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let cam_bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("CamBindGroup"),
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("RenderPipelineLayout"),
        bind_group_layouts: &[&camera_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("RenderPipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[vertex_index_buffer.vbodesc],
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(config.view_formats[0].into())],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            // cull_mode: Some(wgpu::Face::Back),
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: WgpuObject::SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    let msaa_buffer =
        msaa::create_multisampled_framebuffer(&device, &config, WgpuObject::SAMPLE_COUNT);

    let msaa_bundle = msaa::create_bundle(
        &device,
        &config,
        &render_pipeline,
        &vertex_index_buffer.vbo,
        vertex_index_buffer.vbo_size,
    );

    WgpuObject {
        surface,
        device,
        queue,
        config,
        size,
        window,
        pipeline: render_pipeline,
        vertex_buffer: vertex_index_buffer.vbo,
        vertex_buffer_size: vertex_index_buffer.vbo_size,
        index_buffer: vertex_index_buffer.idxbuf,
        index_buffer_size: vertex_index_buffer.idx_size,
        cam: camera,
        cam_bind_group,
        cam_buf: camera_buffer,
        cam_uniform: camera_uniform,
        msaa_buffer,
        msaa_bundle,
        rotation: glam::Vec3::ZERO,
    }
}