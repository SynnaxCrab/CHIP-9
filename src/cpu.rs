use crate::display::{Display, FONT_SET};
use crate::keypad::Keypad;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Cpu {
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
    // random number generator
    rng: ThreadRng,
    // display
    display: Display,
    // keypad
    keypad: Keypad,
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0,
            memory: [0; 4096],
            v: [0; 16],
            stack: [0; 16],
            sp: 0,
            dt: 0,
            rng: rand::thread_rng(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    pub fn set_pixel(&mut self) {
        self.display.set_pixel(0, 0, true);
    }

    pub fn reset(&mut self) {
        self.i = 0;
        self.pc = 0x200;
        self.memory = [0; 4096];
        self.v = [0; 16];
        self.stack = [0; 16];
        self.sp = 0;
        self.dt = 0;
        self.rng = rand::thread_rng();
        self.display.cls();
        for i in 0..80 {
            self.memory[i] = FONT_SET[i];
        }
    }

    pub fn memory_ptr(&self) -> *const u8 {
        self.memory.as_ptr()
    }

    pub fn display_ptr(&self) -> *const u8 {
        self.display.screen.as_ptr()
    }

    pub fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    pub fn current_opcode(&self) -> u16 {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[(self.pc + 1) as usize] as u16;

        hi << 8 | lo
    }

    pub fn process_opcode(&mut self) {
        let opcode = self.current_opcode();
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        // break up into nibbles
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        self.pc += 2;

        match (op_1, op_2, op_3, op_4) {
            // clears the display
            (0, 0, 0xE, 0) => self.display.cls(),
            // returns from a subroutine
            (0, 0, 0xE, 0xE) => {
                self.sp = self.sp - 1;
                self.pc = self.stack[self.sp as usize];
            }
            // Jumps to address
            (0x1, _, _, _) => self.pc = nnn,
            // Calls subroutine
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            // Skips the next instruction if VX equals NN
            (0x3, _, _, _) => {
                if vx == nn {
                    self.pc += 2;
                }
            }
            // Skips the next instruction if VX doesn't equal NN
            (0x4, _, _, _) => {
                if vx != nn {
                    self.pc += 2;
                }
            }
            // Skips the next instruction if VX equals VY
            (0x5, _, _, _) => {
                if vx == vy {
                    self.pc += 2;
                }
            }
            // Set VX to NN
            (0x6, _, _, _) => self.v[x] = nn,
            // Adds NN to VX (Carry flag is not changed)
            (0x7, _, _, _) => self.v[x] += nn,
            // Sets VX to the value of VY
            (0x8, _, _, 0x0) => self.v[x] = self.v[y],
            // Sets VX to VX or VY (Bitwise OR operation)
            (0x8, _, _, 0x1) => self.v[x] = self.v[x] | self.v[y],
            // Sets VX to VX and VY (Bitwise AND operation)
            (0x8, _, _, 0x2) => self.v[x] = self.v[x] & self.v[y],
            // Sets VX to VX xor VY
            (0x8, _, _, 0x3) => self.v[x] = self.v[x] ^ self.v[y],
            // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
            (0x8, _, _, 0x4) => {
                let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
                self.v[0xF] = if overflow { 1 } else { 0 };
                self.v[x] = res;
            }
            // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't
            (0x8, _, _, 0x5) => {
                let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[0xF] = if overflow { 0 } else { 1 };
                self.v[x] = res;
            }
            // Stores the least significant bit of VX in VF and then shifts VX to the right by 1
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            }
            // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't
            (0x8, _, _, 0x7) => {
                let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[0xF] = if overflow { 0 } else { 1 };
                self.v[x] = res;
            }
            // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
            (0x8, _, _, 0xE) => {
                self.v[0xF] = (self.v[x] & 0x80) >> 7;
                self.v[x] <<= 1;
            }
            // Skips the next instruction if VX doesn't equal VY
            (0x9, _, _, _) => {
                if vx != vy {
                    self.pc += 2;
                }
            }
            // Sets I to the address NNN
            (0xA, _, _, _) => self.i = nnn,
            // Jumps to the address NNN plus V0
            (0xB, _, _, _) => self.pc = nnn + self.v[0] as u16,
            // Vx = rand() & NN
            (0xC, _, _, _) => {
                let number = self.rng.gen_range(0, 255);
                self.v[x] = number & nn;
            }
            // Draw the sprite
            (0xD, _, _, _) => {
                let height = n;
                let collision = self.display.draw(
                    vx as usize,
                    vy as usize,
                    &self.memory[self.i as usize..(self.i + height as u16) as usize],
                );
                self.v[0xF] = if collision { 1 } else { 0 };
            }
            // Skips the next instruction if the key stored in VX is pressed.
            // (Usually the next instruction is a jump to skip a code block)
            (0xE, _, 0x9, 0xE) => self.pc += if self.keypad.is_key_down(vx) { 2 } else { 0 },
            // SKips the next instruction if the key stored in VX isn't pressed.
            // (Usually the next instruction is a jump to skip a code block)
            (0xE, _, 0xA, 0x1) => self.pc += if self.keypad.is_key_down(vx) { 0 } else { 2 },
            //Sets VX to the value of the delay timer.
            (0xF, _, 0x0, 0x7) => self.v[x] = self.dt,
            // not implemented yet
            (0xF, _, 0x0, 0xA) => (),
            // Sets the delay timer to VX.
            (0xF, _, 0x1, 0x5) => self.dt = self.v[x],
            // not implemented yet
            (0xF, _, 0x1, 0x8) => (),
            // Adds VX to I. VF is not affected
            (0xF, _, 0x1, 0xE) => self.i = self.i + self.v[x] as u16,
            (_, _, _, _) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;

    #[test]
    fn test_return() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0;
        cpu.memory[1] = 0xEE;
        cpu.pc = 0;
        cpu.sp = 1;
        cpu.stack[0] = 0x655;
        assert_eq!(cpu.current_opcode(), 0x00EE);

        cpu.process_opcode();
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.pc, 0x655);
    }

    #[test]
    fn test_jump() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x16;
        cpu.memory[1] = 0x55;
        cpu.pc = 0;
        assert_eq!(cpu.current_opcode(), 0x1655);

        cpu.process_opcode();
        assert_eq!(cpu.pc, 0x655);
    }

    #[test]
    fn test_call() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x26;
        cpu.memory[1] = 0x55;
        cpu.pc = 0;
        cpu.sp = 0;
        assert_eq!(cpu.current_opcode(), 0x2655);

        cpu.process_opcode();
        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.stack[0], 2);
        assert_eq!(cpu.pc, 0x655);
    }

    #[test]
    fn test_vx_equal() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x30;
        cpu.memory[1] = 0x55;
        cpu.pc = 0;
        cpu.v[0] = 0x55;
        assert_eq!(cpu.current_opcode(), 0x3055);

        cpu.process_opcode();
        assert_eq!(cpu.pc, 4);
    }

    #[test]
    fn test_vx_not_equal() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x40;
        cpu.memory[1] = 0x56;
        cpu.pc = 0;
        cpu.v[0] = 0x55;
        assert_eq!(cpu.current_opcode(), 0x4056);

        cpu.process_opcode();
        assert_eq!(cpu.pc, 4);
    }

    #[test]
    fn test_vx_equals_vy() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x50;
        cpu.memory[1] = 0x10;
        cpu.pc = 0;
        cpu.v[0] = 0x55;
        cpu.v[1] = 0x55;
        assert_eq!(cpu.current_opcode(), 0x5010);

        cpu.process_opcode();
        assert_eq!(cpu.pc, 4);
    }

    #[test]
    fn test_set_vx() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x60;
        cpu.memory[1] = 0x56;
        cpu.pc = 0;
        cpu.v[0] = 0x55;
        assert_eq!(cpu.current_opcode(), 0x6056);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0x56);
    }

    #[test]
    fn test_add_to_vx() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x70;
        cpu.memory[1] = 0x01;
        cpu.pc = 0;
        cpu.v[0] = 0x55;
        assert_eq!(cpu.current_opcode(), 0x7001);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0x56);
    }

    #[test]
    fn test_assign() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x10;
        cpu.pc = 0;
        cpu.v[0] = 0x55;
        cpu.v[1] = 0x66;
        assert_eq!(cpu.current_opcode(), 0x8010);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0x66);
    }

    #[test]
    fn test_bitwise_or() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x11;
        cpu.memory[2] = 0x80;
        cpu.memory[3] = 0x11;
        cpu.pc = 0;
        cpu.v[0] = 0x55;
        cpu.v[1] = 0x00;
        assert_eq!(cpu.current_opcode(), 0x8011);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0x55);

        cpu.v[0] = 0x55;
        cpu.v[1] = 0xFF;
        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0xFF);
    }

    #[test]
    fn test_bitwise_and() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x12;
        cpu.memory[2] = 0x80;
        cpu.memory[3] = 0x12;
        cpu.pc = 0;
        cpu.v[0] = 0x55;
        cpu.v[1] = 0x00;
        assert_eq!(cpu.current_opcode(), 0x8012);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0x00);

        cpu.v[0] = 0x55;
        cpu.v[1] = 0xFF;
        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0x55);
    }

    #[test]
    fn test_bitwise_xor() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x13;
        cpu.memory[2] = 0x80;
        cpu.memory[3] = 0x13;
        cpu.pc = 0;
        cpu.v[0] = 0x00;
        cpu.v[1] = 0x00;
        assert_eq!(cpu.current_opcode(), 0x8013);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0x00);

        cpu.v[0] = 0x00;
        cpu.v[1] = 0xFF;
        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0xFF);
    }

    #[test]
    fn test_add_vy_to_vx() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x14;
        cpu.memory[2] = 0x80;
        cpu.memory[3] = 0x14;
        cpu.pc = 0;
        cpu.v[0] = 0xFE;
        cpu.v[1] = 0x01;
        assert_eq!(cpu.current_opcode(), 0x8014);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0xFF);
        assert_eq!(cpu.v[0xF], 0);

        cpu.v[0] = 0xFF;
        cpu.v[1] = 0x02;
        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0x01);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn test_subtract_vy_from_vx() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x15;
        cpu.memory[2] = 0x80;
        cpu.memory[3] = 0x15;
        cpu.pc = 0;
        cpu.v[0] = 0x02;
        cpu.v[1] = 0x01;
        assert_eq!(cpu.current_opcode(), 0x8015);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0x01);
        assert_eq!(cpu.v[0xF], 1);

        cpu.v[0] = 0x00;
        cpu.v[1] = 0x01;
        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0xFF);
        assert_eq!(cpu.v[0xF], 0);
    }

    #[test]
    fn test_shift_vx_right() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x16;
        cpu.pc = 0;
        cpu.v[0] = 0xFF;
        assert_eq!(cpu.current_opcode(), 0x8016);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0x7F);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn test_set_vx_to_vy_minus_vx() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x17;
        cpu.memory[2] = 0x80;
        cpu.memory[3] = 0x17;
        cpu.pc = 0;
        cpu.v[0] = 0x01;
        cpu.v[1] = 0xFF;
        assert_eq!(cpu.current_opcode(), 0x8017);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0xFE);
        assert_eq!(cpu.v[0xF], 1);

        cpu.v[0] = 0x01;
        cpu.v[1] = 0x00;
        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0xFF);
        assert_eq!(cpu.v[0xF], 0);
    }

    #[test]
    fn test_shift_vx_left() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x1E;
        cpu.pc = 0;
        cpu.v[0] = 0xFF;
        assert_eq!(cpu.current_opcode(), 0x801E);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], 0xFE);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn test_vx_not_equal_vy() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x90;
        cpu.memory[1] = 0x10;
        cpu.pc = 0;
        cpu.v[0] = 0x55;
        cpu.v[0] = 0x54;
        assert_eq!(cpu.current_opcode(), 0x9010);

        cpu.process_opcode();
        assert_eq!(cpu.pc, 4);
    }

    #[test]
    fn test_set_index_register() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0xA5;
        cpu.memory[1] = 0x44;
        cpu.pc = 0;
        assert_eq!(cpu.current_opcode(), 0xA544);

        cpu.process_opcode();
        assert_eq!(cpu.i, 0x544);
    }

    #[test]
    fn test_jump_plus_v0() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0xB5;
        cpu.memory[1] = 0x44;
        cpu.pc = 0;
        cpu.v[0] = 0x1;
        assert_eq!(cpu.current_opcode(), 0xB544);

        cpu.process_opcode();
        assert_eq!(cpu.pc, 0x545);
    }

    #[test]
    fn test_set_vx_to_dt() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0xF0;
        cpu.memory[1] = 0x07;
        cpu.pc = 0;
        cpu.dt = 1;
        assert_eq!(cpu.current_opcode(), 0xF007);

        cpu.process_opcode();
        assert_eq!(cpu.v[0], cpu.dt);
    }

    #[test]
    fn test_set_dt_to_vx() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0xF0;
        cpu.memory[1] = 0x15;
        cpu.pc = 0;
        cpu.v[0] = 1;
        assert_eq!(cpu.current_opcode(), 0xF015);

        cpu.process_opcode();
        assert_eq!(cpu.dt, cpu.v[0]);
    }

    #[test]
    fn test_add_vx_to_i() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0xF0;
        cpu.memory[1] = 0x1E;
        cpu.pc = 0;
        cpu.i = 0xFE;
        cpu.v[0] = 0x1;
        assert_eq!(cpu.current_opcode(), 0xF01E);

        cpu.process_opcode();
        assert_eq!(cpu.i, 0xFF);
    }
}
