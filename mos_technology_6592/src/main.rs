mod memory;
mod cpu;
mod definiton;

use memory::Memory;
use cpu::{CPU, INS_JSR, INS_LDA_IM};

fn main() {
    println!("=== START ===");
    let mut memory = Memory::default();
    let mut cpu = CPU::default();

    cpu.reset(&mut memory);
    memory.data[0xFFFC] = INS_JSR;
    memory.data[0xFFFD] = 0x42;
    memory.data[0xFFFE] = 0x42;
    memory.data[0x4242] = INS_LDA_IM;
    memory.data[0x4243] = 0x84;

    cpu.execute(&mut 9, &mut memory);
    println!("=== END ===");
}
