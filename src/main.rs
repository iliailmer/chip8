struct CPU {
    registers: [u8; 16],
    memory: [u8; 0x1000],
    pos: usize, // position in memory, program counter
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
        //loop {
        let opcode = self.read_op_code();
        let c = ((opcode & 0xF000) >> 12) as u8; // operation code (8 signifies 2-arg operation)
        let x = ((opcode & 0x0F00) >> 8) as u8; // first arg (index in register)
        let y = ((opcode & 0x00F0) >> 4) as u8; // second arg (index in register)
        let d = (opcode & 0x000F) as u8; // the operation code (4 means addition)
        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
        // } //loop
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        // addition
        self.registers[x as usize] += self.registers[y as usize]
    }
}
fn main() {
    let mut cpu = CPU { registers: [0; 16] };
    cpu.current_op = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.run();

    assert_eq!(cpu.registers[0], 15);
}
