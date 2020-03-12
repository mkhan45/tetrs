/// The state of an arbitrary input command
///
/// Key independent
#[derive(Copy, Clone, Default)]
pub struct InputState {
    pressed_frames: u16,
    pressed_this_frame: bool,
}

#[allow(dead_code)]
impl InputState {
    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed_this_frame = pressed;
    }

    /// updates InputState internals
    pub fn update(&mut self) {
        if self.pressed_this_frame {
            self.pressed_this_frame = false;
            self.pressed_frames += 1;
        } else {
            self.pressed_frames = 0;
        }
    }

    pub fn pressed(self) -> bool {
        self.pressed_this_frame
    }

    pub fn held(self) -> bool {
        self.pressed_frames > 1
    }

    pub fn repeated(self, delay: u16, interval: u16) -> bool {
        self.pressed_frames == 1
            || (self.pressed_frames > delay && self.pressed_frames % interval == 0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum InputAction {
    Spin,
    SoftDrop,
    HardDrop,
    MoveLeft,
    MoveRight,
    Cache,
}
