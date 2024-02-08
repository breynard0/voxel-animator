pub mod cam;
pub mod depth;
pub mod init;
pub mod input;
pub mod lines;
pub mod msaa;
pub mod render;
pub mod texture;
pub mod transform;
pub mod vertex;
pub mod wgpu_object;

use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};

use crate::utils::log::log;
pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    let window = WindowBuilder::new()
        .with_title("Voxel Animator")
        .with_decorations(true)
        .build(&event_loop)
        .unwrap();
    let window_id = window.id();

    let mut wgpu_obj = init::gfx_init(&window).await;

    let mut last_frame = std::time::Instant::now();

    match event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            ref event,
            window_id: queried_window_id,
        } if queried_window_id == window_id => match event {
            WindowEvent::RedrawRequested if window_id == wgpu_obj.window().id() => {
                wgpu_obj.update();
                match render::render(&mut wgpu_obj) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => wgpu_obj.resize(wgpu_obj.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        log(
                            "Not enough memory to map the next frame!",
                            crate::utils::log::LogLevel::ERROR,
                        );
                    }
                    Err(e) => log(
                        format!("{}", e.to_string()),
                        crate::utils::log::LogLevel::ERROR,
                    ),
                }
            }
            WindowEvent::KeyboardInput { event, .. } => input::poll_keyboard_event(event),
            WindowEvent::MouseInput { button, state, .. } => {
                input::poll_mousebutton_event(button, state)
            }
            WindowEvent::MouseWheel { delta, .. } => input::poll_scroll_wheel(delta),
            WindowEvent::Resized(physical_size) => {
                wgpu_obj.resize(*physical_size);
            }
            WindowEvent::CursorMoved { position, .. } => input::poll_mouse_move_event(position),
            WindowEvent::CloseRequested => elwt.exit(),
            _ => {}
        },
        Event::AboutToWait => {
            wgpu_obj.delta_time = std::time::Instant::now()
                .duration_since(last_frame)
                .as_secs_f32();
            last_frame = std::time::Instant::now();
            wgpu_obj.window().request_redraw();
        }
        _ => {}
    }) {
        Ok(_) => {}
        Err(e) => log(e.to_string(), crate::utils::log::LogLevel::FATAL),
    }
}
