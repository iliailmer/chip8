//TODO: Implement multiplication
mod cpu;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu {
        registers: [0; 16],
        register_i: 0,
        memory: [0; 4096],
        prog_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // op code 0x2100, call function at addr 100
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    // // op code 0x2100, call function at addr 100
    // mem[0x002] = 0x21;
    // mem[0x003] = 0x00;
    // // op code 0x0000, halt
    // mem[0x004] = 0x00;
    // mem[0x005] = 0x00;

    mem[0x100] = 0x80;
    mem[0x101] = 0x1E;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;
    cpu.run();

    // assert_eq!(cpu.registers[0], 45);
    println!("{}", cpu.registers[0]);
    println!("{}", cpu.registers[0xF]);
}
