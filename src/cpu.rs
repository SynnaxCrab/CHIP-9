struct Cpu {
    // index register
    i: u16,
    // program counter
    pc: u16,
    // memory
    memory: [u8; 4096],
    // registers
    v: [u8; 16],
    // stack
    stack: [u16; 16],
    // stack pointer
    sp: u8,
    // delayed timer
    dt: u8,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0,
            memory: [0; 4096],
            v: [0: 16],
            stack: [0, 16],
            sp: 0,
            dt: 0
        }
    }

    fn current_opcode() -> u16 {
        let hi = self.memory[pc] as u16;
        let lo = self.memory[pc + 1] as u16;

        hi << 8 | lo
    }
}
