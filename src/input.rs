use winit::event::*;

pub struct ReadOnlyInput {
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
}

impl ReadOnlyInput {
    pub fn new() -> Self {
        Self {
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
        }
    }

    pub fn is_down_pressed(&self) -> bool {
        self.is_down_pressed
    }

    pub fn is_left_pressed(&self) -> bool {
        self.is_left_pressed
    }

    pub fn is_right_pressed(&self) -> bool {
        self.is_right_pressed
    }

    pub fn is_up_pressed(&self) -> bool {
        self.is_up_pressed
    }
}

pub struct Input {
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
        }
    }

    pub fn to_read_only(&self) -> ReadOnlyInput {
        ReadOnlyInput {
            is_down_pressed: self.is_down_pressed,
            is_left_pressed: self.is_left_pressed,
            is_right_pressed: self.is_right_pressed,
            is_up_pressed: self.is_up_pressed,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;

                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}
