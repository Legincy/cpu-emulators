use crate::memory::Memory;
use crate::definiton::{Byte, Word};

pub const INS_LDA_IM: Byte = 0xA9;
pub const INS_LDA_ZP: Byte = 0xA5;
pub const INS_LDA_ZPX: Byte = 0xB5;
pub const INS_JSR: Byte = 0x20;

#[derive(Debug)]
pub struct CPU{
    pc: Word,   // Program Counter
    sp: Byte,   // Stack Pointer

    a: Byte,    // Accumulator
    x: Byte,    // Index Register X
    y: Byte,    // Index Register Y

    c: Byte,    // Carry Flag
    z: Byte,    // Zero Flag
    i: Byte,    // Interrupt Disable
    d: Byte,    // Decimal Mode
    b: Byte,    // Break Command
    v: Byte,    // Overflow Flag
    n: Byte,    // Negative Flag
}

impl CPU {
    pub fn default() -> CPU {
        CPU {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,

            c: 1,
            z: 1,
            i: 1,
            d: 1,
            b: 1,
            v: 1,
            n: 1,
        }
    }

    pub fn reset(&mut self, memory: &mut Memory) {
        self.pc = 0xFFFC;
        self.sp = 0xFF;
        self.a = 0;
        self.x = 0;
        self.y = 0;

        self.c = 0;
        self.z = 0;
        self.i = 0;
        self.d = 0;
        self.b = 0;
        self.v = 0;
        self.n = 0;

        memory.init();
    }

    pub fn execute(&mut self, cycles: &mut u32, memory: &mut Memory) {
        while *cycles > 0 {
            let instruction = self.fetch_byte(cycles, memory, true, None);
            println!("instruction: {:#X}, memory: {:#X}, cycles: {}", instruction, memory.read(self.pc), cycles);

            match instruction {
                INS_LDA_IM => {
                    let value = self.fetch_byte(cycles, memory, true, None);
                    self.a = value;
                    self.set_status_after_load()
                }
                INS_LDA_ZP => {
                    let zero_page_addr = self.fetch_byte(cycles, memory, false, None);
                    self.a = self.fetch_byte(cycles, memory, true, Some(zero_page_addr as Word));
                    self.set_status_after_load()
                }
                INS_LDA_ZPX => {
                    let mut zero_page_addr = self.fetch_byte(cycles, memory, false, None);
                    zero_page_addr += self.x;
                    *cycles -= 1;
                    self.a = self.fetch_byte(cycles, memory, true, Some(zero_page_addr as Word));
                    self.set_status_after_load()

                }
                INS_JSR => {
                    let sub_routine_addr = self.fetch_word(cycles, memory);
                    self.sp -= 1;
                    memory.write_word(cycles, self.sp as u16, self.pc - 1);
                    self.pc = sub_routine_addr;
                    *cycles -= 1;
                }
                _ => {
                    println!("Instruction not handled: {:#X}", instruction);
                }
            }
        }
    }

    pub fn fetch_byte(&mut self, cycles: &mut u32, memory: &Memory, increase_pc: bool, address: Option<Word>) -> Byte {
        let mem_addr = address.unwrap_or(self.pc);
        let data =  memory.read(mem_addr);

        if increase_pc { self.pc += 1; }
        *cycles -= 1;

        data
    }

    pub fn fetch_word(&mut self, cycles: &mut u32, memory: &Memory) -> Word {
        let low_byte = self.fetch_byte(cycles, memory, true, None) as Word;
        let high_byte = self.fetch_byte(cycles, memory, true, None) as Word;
        
        (high_byte << 8) | low_byte
    }
    
    fn set_status_after_load (&mut self) {
        self.z = (self.a == 0) as u8;
        self.n = (self.a & 0b1000_0000 > 0) as u8;
    }
}
