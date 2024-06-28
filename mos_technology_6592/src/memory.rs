use crate::definiton::{Byte, Word};

pub struct Memory {
    pub data: [Byte; Memory::MEMORY_SIZE]
}

impl Memory {
    const MEMORY_SIZE: usize = 1024 * 64;

    pub fn default() -> Memory {
        Memory {
            data: [0; Memory::MEMORY_SIZE]
        }
    }

    pub fn init(&mut self) {
        for i in 0..Memory::MEMORY_SIZE {
            self.data[i] = 0;
        }
    }

    pub fn read(&self, address: u16) -> Byte {
        self.data[address as usize]
    }

    pub fn write_word(&mut self, cycles: &mut u32, address: u16, value: Word) {
        self.data[address as usize] = (value & 0xFF) as Byte;
        self.data[(address + 1) as usize] = (value >> 8) as Byte;
        *cycles -= 2;
    }
}
