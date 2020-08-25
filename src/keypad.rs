struct Keypad {
    keys: [bool; 16];
}

impl Keypad {
    fn new() -> Keypad {
        Keypad {
            keys: [false; 16]
        }
    }

    fn key_down(&mut self, index: u8) {
        self.keys[index as usize] = true;
    }

    pub fn key_up(&mut self, index: u8) {
        self.keys[index as usize] = false;
    }
}
