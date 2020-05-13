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

    fn process_opcode(&mut self) {
        let x = (current_opcode() & 0x0F00) >> 8;
        let y = (current_opcode() & 0x00F0) >> 4;
        let vx = self.v[x];
        let vy = self.v[y];
        let nnn = opcode & 0x0FFF;
        let nn = opcode & 0x00FF;

        // break up into nibbles
        let op_1 = (current_opcode() & 0xF000) >> 12;
        let op_2 = (current_opcode() & 0x0F00) >> 8;
        let op_3 = (current_opcode() & 0x00F0) >> 4;
        let op_4 = current_opcode() & 0x000F

        self.pc += 2;

        match (op_1, op_2, op_3, op_4) {
            // clears the display
            (0, 0, 0xE, 0) => self.display.cls(),
            // returns from a subroutine
            (0, 0, 0xE, 0xE) => {
                self.sp = self.sp - 1;
                self.pc = self.stack[self.sp as usize];
            },
            // Jumps to address
            (0x1, _, _, _) => self.pc = nnn,
            // Calls subroutine
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            // Skips the next instruction if VX equals NN
            (0x3, _, _, _) => {
                if vx == nn {
                    self.pc += 2;
                }
            },
            // Skips the next instruction if VX doesn't equal NN
            (0x4, _, _, _) => {
                if vx != nn {
                    self.pc += 2;
                }
            },
            // Skips the next instruction if VX equals VY
            (0x5, _, _, _) => {
                if vx = vy {
                    self.pc += 2;
                }
            },
            // Set VX to NN
            (0x6, _, _, _) => self.v[x] = nn,
            // Adds NN to VX (Carry flag is not changed)
            (0x7, _, _, _) => self.v[x] += nn,
            // Sets VX to the value of VY.
            (0x8, _, _, 0x1) => self.v[x] = v[y],
            (0xA, _, _, _) => self.i = nnn,
        }
    }

}
