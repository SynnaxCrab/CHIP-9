struct Keypad {
    keys: [bool; 16];
}

impl Keypad {
    fn new() -> Keypad {
        Keypad {
            keys: [false; 16]
        }
    }
}
