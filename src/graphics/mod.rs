pub mod cam;
pub mod init;
pub mod input;
pub mod msaa;
pub mod render;
pub mod vertex;
pub mod wgpu_object;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Voxel Animator")
        .with_decorations(true)
        .build(&event_loop)
        .unwrap();
    let window_id = window.id();

    let mut wgpu_obj = init::gfx_init(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id: queried_window_id,
        } if queried_window_id == window_id => match event {
            WindowEvent::KeyboardInput { input, .. } => input::poll_keyboard_event(input),
            WindowEvent::MouseInput { button, state, .. } => {
                input::poll_mousebutton_event(button, state)
            }
            WindowEvent::Resized(physical_size) => {
                wgpu_obj.resize(*physical_size);
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == wgpu_obj.window().id() => {
            wgpu_obj.update();
            match render::render(&mut wgpu_obj) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => wgpu_obj.resize(wgpu_obj.size),
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    log::error!("Not enough memory to map the next frame!")
                }
                Err(e) => log::error!("{}", e.to_string()),
            }
        }
        Event::MainEventsCleared => {
            wgpu_obj.window().request_redraw();
        }
        _ => {}
    });
}
