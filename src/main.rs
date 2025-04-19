struct CPU {
    registers: [u8; 16],
    memory: [u8; 0x1000],
    pos: usize, // position in memory, program counter
    stack: [u16; 16],
    stack_pointer: usize,
}
impl CPU {
    fn read_op_code(&self) -> u16 {
        //NOTE: listing 5.24
        let p = self.pos;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_op_code();
            self.pos += 2;
            let nnn = opcode & 0x0FFF; // address for function call: 0x2nnn

            let c = ((opcode & 0xF000) >> 12) as u8; // operation code (8 signifies 2-arg operation)
            let x = ((opcode & 0x0F00) >> 8) as u8; // first arg (index in register)
            let y = ((opcode & 0x00F0) >> 4) as u8; // second arg (index in register)
            let d = (opcode & 0x000F) as u8; // the operation code (4 means addition)
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.rtrn(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        } //loop
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

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        // let stack = &mut self.stack;
        if sp > self.stack.len() {
            panic!("Stack overflow");
        }
        self.stack[sp] = self.pos as u16;
        self.stack_pointer += 1;
        self.pos = addr as usize;
    }
    fn rtrn(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow")
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.pos = call_addr as usize;
    }
}
fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        pos: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // op code 0x0814, addition (4) of reg 1 to reg 0
    mem[0] = 0x80;
    mem[1] = 0x14;
    // op code 0x0824, addition (4) of reg 2 to reg 0
    mem[2] = 0x80;
    mem[3] = 0x24;
    // op code 0x0834, addition (4) of reg 3 to reg 0
    mem[4] = 0x80;
    mem[5] = 0x34;
    cpu.run();

    assert_eq!(cpu.registers[0], 35);
}
