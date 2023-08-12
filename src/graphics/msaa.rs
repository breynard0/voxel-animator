use wgpu::RenderBundleEncoder;

use super::wgpu_object::WgpuObject;

pub fn create_multisampled_framebuffer(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    sample_count: u32,
) -> wgpu::TextureView {
    let multisampled_texture_extent = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1,
    };

    let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
        size: multisampled_texture_extent,
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: config.view_formats[0],
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        label: Some("MSAAFrameDescriptor"),
        view_formats: &[],
    };

    device
        .create_texture(multisampled_frame_descriptor)
        .create_view(&wgpu::TextureViewDescriptor::default())
}

pub fn create_bundle(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    pipeline: &wgpu::RenderPipeline,
    vertex_buffer: &wgpu::Buffer,
    vertex_buffer_size: u32,
) -> wgpu::RenderBundle {
    let mut encoder = device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
        label: None,
        color_formats: &[Some(config.view_formats[0])],
        depth_stencil: None,
        sample_count: WgpuObject::SAMPLE_COUNT,
        multiview: None,
    });
    encoder.set_pipeline(&pipeline);
    encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
    encoder.draw(0..vertex_buffer_size, 0..1);
    encoder.finish(&wgpu::RenderBundleDescriptor {
        label: Some("MSAA Render Bundle"),
    })
}

// pub fn update_bundle_vbo(
//     encoder: &mut RenderBundleEncoder<'_>
// )
