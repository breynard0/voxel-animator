use winit::event::{ElementState, VirtualKeyCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMouseButton {
    Left,
    Right,
    Middle,
}

static mut KEY_STATE: Vec<VirtualKeyCode> = vec![];
static mut LAST_KEY_STATE: Vec<VirtualKeyCode> = vec![];

static mut MOUSE_STATE: Vec<InputMouseButton> = vec![];
static mut LAST_MOUSE_STATE: Vec<InputMouseButton> = vec![];

pub fn poll_keyboard_event(event: &winit::event::KeyboardInput) {
    let key = match event.virtual_keycode {
        Some(k) => k,
        None => {
            log::info!("Unable to parse key");
            return;
        }
    };

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
        winit::event::MouseButton::Other(_) => {
            log::warn!("Auxiliary mouse buttons are currently unsupported")
        }
    }
}

pub fn input_update() {
    unsafe { LAST_MOUSE_STATE = MOUSE_STATE.clone() }
    unsafe { LAST_KEY_STATE = KEY_STATE.clone() }
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

pub fn is_key_down(key: VirtualKeyCode) -> bool {
    unsafe { KEY_STATE.contains(&key) }
}

pub fn is_mouse_button_down(button: InputMouseButton) -> bool {
    unsafe { MOUSE_STATE.contains(&button) }
}

pub fn is_key_pressed(key: VirtualKeyCode) -> bool {
    unsafe { !LAST_KEY_STATE.contains(&key) && KEY_STATE.contains(&key) }
}

pub fn is_key_released(key: VirtualKeyCode) -> bool {
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
