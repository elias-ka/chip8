use sdl2::keyboard::Scancode;

pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn set_key(&mut self, key: Scancode, pressed: bool) {
        match key {
            Scancode::Num1 => self.keys[0x1] = pressed,
            Scancode::Num2 => self.keys[0x2] = pressed,
            Scancode::Num3 => self.keys[0x3] = pressed,
            Scancode::Num4 => self.keys[0xC] = pressed,
            Scancode::Q => self.keys[0x4] = pressed,
            Scancode::W => self.keys[0x5] = pressed,
            Scancode::E => self.keys[0x6] = pressed,
            Scancode::R => self.keys[0xD] = pressed,
            Scancode::A => self.keys[0x7] = pressed,
            Scancode::S => self.keys[0x8] = pressed,
            Scancode::D => self.keys[0x9] = pressed,
            Scancode::F => self.keys[0xE] = pressed,
            Scancode::Z => self.keys[0xA] = pressed,
            Scancode::X => self.keys[0x0] = pressed,
            Scancode::C => self.keys[0xB] = pressed,
            Scancode::V => self.keys[0xF] = pressed,
            _ => {}
        }
    }

    pub fn wait_for_key(&self) -> Option<u8> {
        for (i, key) in self.keys.iter().enumerate() {
            if *key {
                return Some(i as u8);
            }
        }

        None
    }
}
