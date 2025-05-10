use rand::prelude::*;
use std::ops::{BitAnd, BitOr, BitXor};

pub struct Cpu {
    pub registers: [u8; 16],
    pub register_i: u16,
    pub memory: [u8; 0x1000],
    pub prog_counter: usize, // prog_counterition in memory, program counter
    pub stack: [u16; 16],
    pub stack_pointer: usize,
}

#[allow(dead_code, unused_variables)]
impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            register_i: 0,
            memory: [0; 4096],
            prog_counter: 0x200, // programs start at 0x200
            stack: [0; 16],
            stack_pointer: 0,
        }
    }
    fn read_op_code(&self) -> u16 {
        let p = self.prog_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_op_code();
            self.prog_counter += 2;
            let nnn = opcode & 0x0FFF; // address for function call: 0x2nnn
            let kk = (opcode & 0x00FF) as u8;
            let c = ((opcode & 0xF000) >> 12) as u8; // operation code (8 signifies 2-arg operation)
            let x = ((opcode & 0x0F00) >> 8) as u8; // first arg (index in register)
            let y = ((opcode & 0x00F0) >> 4) as u8; // second arg (index in register)
            let d = (opcode & 0x000F) as u8; // the operation code (4 means addition)
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.rtrn(),
                (0x1, _, _, _) => self.jump(nnn),
                (0x2, _, _, _) => self.call(nnn),
                (0x3, _, _, _) => self.skip_e(x, kk),
                (0x4, _, _, _) => self.skip_ne(x, kk),
                (0x5, _, _, 0x0) => self.skip_e_xy(x, y),
                (0x6, _, _, _) => self.ld_xkk(x, kk),
                (0x7, _, _, _) => self.add_xkk(x, kk),
                (0x8, _, _, 0x0) => self.set_xy(x, y),
                (0x8, _, _, 0x1) => self.or_xy(x, y),
                (0x8, _, _, 0x2) => self.and_xy(x, y),
                (0x8, _, _, 0x3) => self.xor_xy(x, y),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0x8, _, _, 0x5) => self.sub_xy(x, y),
                (0x8, _, _, 0x6) => self.shr(x),
                (0x8, _, _, 0x7) => self.sub_yx(x, y),
                (0x8, _, _, 0xE) => self.shl(x),
                (0x9, _, _, _) => self.skip_ne(x, y),
                (0xA, _, _, _) => self.set_i_nnn(nnn),
                (0xB, _, _, _) => self.add_reg0_nnn(nnn),
                (0xC, _, _, _) => self.rand_byte_and_xkk(x, kk),
                (0xD, _, _, _) => self.d_xyn(x, y, d),
                (0xE, _, 0x9, 0xE) => self.e_x9e_skip_pressed_x(x),
                (0xE, _, 0xA, 0x1) => self.e_xa1_skip_npressed_x(x),
                (0xF, _, 0x0, 0x7) => self.f_x07_set_delay_x(x),
                (0xF, _, 0x0, 0xA) => self.wait_keypress_x(x),
                (0xF, _, 0x1, 0x5) => self.set_delay_timer(x),
                (0xF, _, 0x1, 0x8) => self.set_sound_timer(x),
                (0xF, _, 0x1, 0xE) => self.add_i_x(x),
                (0xF, _, 0x2, 0x9) => self.fx29_set_i_sprite(x),
                (0xF, _, 0x3, 0x3) => self.fx33(x),
                (0xF, _, 0x5, 0x5) => self.fx55(x),
                (0xF, _, 0x6, 0x5) => self.fx65(x),
                // NOTE: skipping drawing instructions
                _ => todo!("opcode {:04x}", opcode),
            }
        } //loop
    }

    fn jump(&mut self, addr: u16) {
        self.prog_counter = addr as usize;
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        // let stack = &mut self.stack;
        if sp > self.stack.len() {
            panic!("Stack overflow");
        }
        self.stack[sp] = self.prog_counter as u16;
        self.stack_pointer += 1;
        self.prog_counter = addr as usize;
    }

    fn skip_e(&mut self, x: u8, addr: u8) {
        if self.registers[x as usize] == addr {
            self.prog_counter += 2;
        }
    }

    fn skip_ne(&mut self, x: u8, addr: u8) {
        if self.registers[x as usize] != addr {
            self.prog_counter += 2;
        }
    }

    fn skip_e_xy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.prog_counter += 2;
        }
    }

    fn ld_xkk(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = kk;
    }

    fn add_xkk(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] += kk;
    }

    fn set_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize].bitor(self.registers[y as usize]);
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize].bitand(self.registers[y as usize]);
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize].bitxor(self.registers[y as usize]);
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        // addition
        let arg_1 = self.registers[x as usize];
        let arg_2 = self.registers[y as usize];
        let (val, overflow) = arg_1.overflowing_add(arg_2);
        self.registers[x as usize] = val;
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];
        if vx > vy {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[x as usize] = vx - vy;
    }

    fn sub_yx(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];
        if vy > vx {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[x as usize] = vy - vx;
    }

    fn shr(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        self.registers[0xF] = vx & 0x1;
        self.registers[x as usize] = vx >> 1;
    }

    fn shl(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        self.registers[0xF] = (vx >> 7) & 0x1;
        self.registers[x as usize] = vx << 1;
    }

    fn rtrn(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow")
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.prog_counter = call_addr as usize;
    }

    fn sne_xy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] != self.registers[y as usize] {
            self.prog_counter += 2;
        }
    }

    fn set_i_nnn(&mut self, nnn: u16) {
        self.register_i = nnn;
    }

    fn add_reg0_nnn(&mut self, nnn: u16) {
        self.prog_counter = (self.registers[0] as u16 + nnn) as usize;
    }

    fn rand_byte_and_xkk(&mut self, x: u8, kk: u8) {
        let mut rng = rand::rng();
        let random_byte = rng.random_range(0..255);
        self.registers[x as usize] = random_byte.bitand(kk);
    }

    fn d_xyn(&mut self, x: u8, y: u8, n: u8) {
        todo!("Not Implemented")
    }
    fn e_x9e_skip_pressed_x(&mut self, x: u8) {
        todo!("Not implemented")
    }
    fn e_xa1_skip_npressed_x(&mut self, x: u8) {
        todo!("Not implemented")
    }
    fn f_x07_set_delay_x(&mut self, x: u8) {
        todo!("Not implemented")
    }
    fn wait_keypress_x(&mut self, x: u8) {
        todo!("Not implemented")
    }
    fn set_delay_timer(&mut self, x: u8) {
        todo!("Not implemented")
    }
    fn set_sound_timer(&mut self, x: u8) {
        todo!("Not implemented")
    }
    fn add_i_x(&mut self, x: u8) {
        todo!("Not implemented")
    }
    fn fx29_set_i_sprite(&mut self, x: u8) {
        todo!("Not implemented")
    }
    fn fx33(&mut self, x: u8) {
        todo!("Not implemented")
    }
    fn fx55(&mut self, x: u8) {
        todo!("Not implemented")
    }
    fn fx65(&mut self, x: u8) {
        todo!("Not implemented")
    }
}
