use wgpu::SurfaceError;

use super::wgpu_object::WgpuObject;

pub fn render(wobj: &mut WgpuObject) -> Result<(), SurfaceError> {
    let output = wobj.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some("RenderTexture View"),
        format: Some(wobj.config.view_formats[0]),
        dimension: Some(wgpu::TextureViewDimension::D2),
        aspect: wgpu::TextureAspect::All,
        ..Default::default()
    });

    let color_attachment = match WgpuObject::SAMPLE_COUNT {
        1 => wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        },
        _ => wgpu::RenderPassColorAttachment {
            view: &wobj.msaa_buffer,
            resolve_target: Some(&view),
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Discard,
            },
        },
    };

    let mut encoder = wobj
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("commandencoder"),
        });

    // Copy camera info if it is updated
    match &wobj.cam_staging_buf {
        Some(b) => {
            encoder.copy_buffer_to_buffer(b, 0, &wobj.cam_buf, 0, b.size());
        }
        None => {}
    }
    wobj.cam_staging_buf = None;

    // Copy transform info if it is updated
    match &wobj.transform_staging_buf {
        Some(b) => {
            encoder.copy_buffer_to_buffer(b, 0, &wobj.transform_buf, 0, b.size());
        }
        None => {}
    }
    wobj.transform_staging_buf = None;

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &wobj.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.execute_bundles(std::iter::once(&wobj.msaa_bundle));

        render_pass.set_pipeline(&wobj.pipeline);
        render_pass.set_bind_group(0, &wobj.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, wobj.vertex_buffer.slice(..));
        render_pass.set_index_buffer(wobj.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..wobj.index_buffer_size, 0, 0..1);
    }

    wobj.queue.submit(std::iter::once(encoder.finish()));

    output.present();

    Ok(())
}
