pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { keys: [false; 16] }
    }

    fn key_down(&mut self, index: u8) {
        self.keys[index as usize] = true;
    }

    fn key_up(&mut self, index: u8) {
        self.keys[index as usize] = false;
    }

    pub fn is_key_down(&self, index: u8) -> bool {
        self.keys[index as usize]
    }
}
