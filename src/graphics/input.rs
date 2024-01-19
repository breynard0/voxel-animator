use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseScrollDelta},
    keyboard::KeyCode,
};

use crate::utils::log;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMouseButton {
    Left,
    Right,
    Middle,
}

// I know this looks terrible, but I promise it's safe

static mut KEY_STATE: Vec<KeyCode> = vec![];
static mut LAST_KEY_STATE: Vec<KeyCode> = vec![];

static mut MOUSE_STATE: Vec<InputMouseButton> = vec![];
static mut LAST_MOUSE_STATE: Vec<InputMouseButton> = vec![];

static mut MOUSE_POS: PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0);
static mut LAST_MOUSE_POS: PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0);

static mut MOUSE_SCROLL_LAST: f32 = 0.0;

static mut SHIFT_DOWN: bool = false;
static mut CTRL_DOWN: bool = false;
static mut ALT_DOWN: bool = false;

pub fn poll_keyboard_event(event: &winit::event::KeyEvent) {
    let key = match event.physical_key {
        winit::keyboard::PhysicalKey::Code(k) => k,
        winit::keyboard::PhysicalKey::Unidentified(k) => {
            log::log(
                format!("Unable to parse key with code {:?}", k),
                log::LogLevel::WARNING,
            );
            return;
        }
    };

    // Check for modifiers
    let mut alt_down = false;
    if let winit::keyboard::KeyCode::AltLeft = key {
        alt_down = true;
    }
    if let winit::keyboard::KeyCode::AltRight = key {
        alt_down = true;
    }
    unsafe {
        ALT_DOWN = match alt_down {
            true => event.state.is_pressed(),
            false => ALT_DOWN,
        }
    }

    let mut ctrl_down = false;
    if let winit::keyboard::KeyCode::ControlLeft = key {
        ctrl_down = true;
    }
    if let winit::keyboard::KeyCode::ControlRight = key {
        ctrl_down = true;
    }
    unsafe {
        CTRL_DOWN = match ctrl_down {
            true => event.state.is_pressed(),
            false => CTRL_DOWN,
        };
    }
    let mut shift_down = false;
    if let winit::keyboard::KeyCode::ShiftLeft = key {
        shift_down = true;
    }
    if let winit::keyboard::KeyCode::ShiftRight = key {
        shift_down = true;
    }

    unsafe {
        SHIFT_DOWN = match shift_down {
            true => event.state.is_pressed(),
            false => SHIFT_DOWN,
        };
    }
    unsafe {
        match event.state {
            ElementState::Pressed => {
                if !KEY_STATE.contains(&key) {
                    KEY_STATE.push(key);
                }
            }
            ElementState::Released => {
                if KEY_STATE.contains(&key) {
                    KEY_STATE.remove(KEY_STATE.iter().position(|x| x == &key).unwrap());
                }
            }
        }
    }
}

pub fn poll_mousebutton_event(event: &winit::event::MouseButton, state: &ElementState) {
    match event {
        winit::event::MouseButton::Left => mousebutton_ops(InputMouseButton::Left, state),
        winit::event::MouseButton::Right => mousebutton_ops(InputMouseButton::Right, state),
        winit::event::MouseButton::Middle => mousebutton_ops(InputMouseButton::Middle, state),
        winit::event::MouseButton::Back => todo!(),
        winit::event::MouseButton::Forward => todo!(),
        winit::event::MouseButton::Other(_) => {
            log::log(
                "Certain mouse buttons are currently unsupported",
                log::LogLevel::WARNING,
            );
        }
    }
}

pub fn poll_mouse_move_event(position: &winit::dpi::PhysicalPosition<f64>) {
    unsafe { MOUSE_POS = position.clone() }
}

pub fn poll_scroll_wheel(delta: &MouseScrollDelta) {
    unsafe {
        MOUSE_SCROLL_LAST = match delta {
            MouseScrollDelta::LineDelta(_, y) => *y,
            MouseScrollDelta::PixelDelta(x) => x.y as f32,
        }
    }
}

pub fn reset_scroll_wheel() {
    unsafe { MOUSE_SCROLL_LAST = 0.0 }
}

pub fn input_update() {
    unsafe { LAST_MOUSE_STATE = MOUSE_STATE.clone() }
    unsafe { LAST_KEY_STATE = KEY_STATE.clone() }
    unsafe { LAST_MOUSE_POS = MOUSE_POS.clone() }
    reset_scroll_wheel();
}

fn mousebutton_ops(button: InputMouseButton, state: &ElementState) {
    let mut mousestate = unsafe { MOUSE_STATE.clone() };

    if matches!(state, ElementState::Released) && mousestate.contains(&button) {
        mousestate.remove(mousestate.iter().position(|x| x == &button).unwrap());
    }

    if matches!(state, ElementState::Pressed) && !mousestate.contains(&button) {
        mousestate.push(button);
    }

    unsafe { MOUSE_STATE = mousestate }
}

pub fn is_key_down(key: KeyCode) -> bool {
    unsafe { KEY_STATE.contains(&key) }
}

pub fn is_mouse_button_down(button: InputMouseButton) -> bool {
    unsafe { MOUSE_STATE.contains(&button) }
}

pub fn is_key_pressed(key: KeyCode) -> bool {
    unsafe { !LAST_KEY_STATE.contains(&key) && KEY_STATE.contains(&key) }
}

pub fn is_key_released(key: KeyCode) -> bool {
    unsafe { LAST_KEY_STATE.contains(&key) && !KEY_STATE.contains(&key) }
}

pub fn is_mouse_pressed(key: InputMouseButton) -> bool {
    unsafe { !LAST_MOUSE_STATE.contains(&key) && MOUSE_STATE.contains(&key) }
}

pub fn is_mouse_released(key: InputMouseButton) -> bool {
    unsafe { LAST_MOUSE_STATE.contains(&key) && !MOUSE_STATE.contains(&key) }
}

// Debugging function
pub fn log_key_state() {
    unsafe { println!("{:?}", KEY_STATE) }
}

pub fn get_mouse_position() -> PhysicalPosition<f64> {
    unsafe { MOUSE_POS }
}

pub fn get_mouse_delta() -> PhysicalPosition<f64> {
    unsafe {
        PhysicalPosition::new(
            MOUSE_POS.x - LAST_MOUSE_POS.x,
            MOUSE_POS.y - LAST_MOUSE_POS.y,
        )
    }
}

/// Returns mouse position between -1 and 1 on both axes
pub fn get_mouse_position_range(window_size: PhysicalSize<u32>) -> (f32, f32) {
    let pos = get_mouse_position();

    let x_range = pos.x / window_size.width as f64;
    let y_range = pos.y / window_size.height as f64;

    (x_range as f32 * 2.0 - 1.0, y_range as f32 * 2.0 - 1.0)
}

/// Returns mouse delta between -1 and 1 on both axes
pub fn get_mouse_delta_range(window_size: PhysicalSize<u32>) -> (f32, f32) {
    let delta = get_mouse_delta();

    let x_range = delta.x / window_size.width as f64;
    let y_range = delta.y / window_size.height as f64;

    (x_range as f32, y_range as f32)
}

pub fn get_scroll_delta() -> f32 {
    unsafe { MOUSE_SCROLL_LAST }
}

pub fn is_shift_down() -> bool {
    unsafe { SHIFT_DOWN }
}

pub fn is_ctrl_down() -> bool {
    unsafe { CTRL_DOWN }
}

pub fn is_alt_down() -> bool {
    unsafe { ALT_DOWN }
}
