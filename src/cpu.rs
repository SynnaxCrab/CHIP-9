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
            // Sets VX to the value of VY
            (0x8, _, _, 0x0) => self.v[x] = v[y],
            // Sets VX to VX or VY (Bitwise OR operation)
            (0x8, _, _, 0x1) => self.v[x] = self.v[x] | self.v[y],
            // Sets VX to VX and VY (Bitwise AND operation)
            (0x8, _, _, 0x2) => self.v[x] = self.v[x] & self.v[y],
            // Sets VX to VX xor VY
            (0x8, _, _, 0x3) => self.v[x] = self.v[x] ^ self.v[y],
            // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
            (0x8, _, _, 0x4) => self.v[x] = self.v[x] ^ self.v[y],
            // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
            (0x8, _, _, 0x4) => {
                let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
                self.v[0xF] = if overflow { 1 } else { 0 };
                self.v[x] = res;
            },
            // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't
            (0x8, _, _, 0x5) => {
                let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[0xF] = if overflow { 0 } else { 1 };
                self.v[x] = res;
            },
            // Stores the least significant bit of VX in VF and then shifts VX to the right by 1
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            },
            // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't
            (0x8, _, _, 0x7) => {
                let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[0xF] = if overflow { 0 } else { 1 };
                self.v[x] = res;
            },
            // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
            (0x8, _, _, 0xE) => {
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            },
            // Skips the next instruction if VX doesn't equal VY
            (0x9, _, _, _) => {
                if vx != vy {
                    self.pc += 2;
                }
            },
            // Sets I to the address NNN
            (0xA, _, _, _) => self.i = nnn,
            // Jumps to the address NNN plus V0
            (0xB, _, _, _) => self.pc = nnn + self.v[0],
        }
    }

}
