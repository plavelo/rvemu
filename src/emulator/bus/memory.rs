use crate::emulator::bus::Size;

pub const MEMORY_SIZE: u64 = 1024 * 1024 * 1024;
pub const MEMORY_BASE_ADDRESS: u64 = 0x8000_0000;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE as usize],
        }
    }
}

impl Memory {
    pub fn size(&self) -> u64 {
        MEMORY_BASE_ADDRESS + MEMORY_SIZE
    }

    pub fn load(&self, address: u64, size: Size) -> u64 {
        (0..size as usize).fold(0, |acc, i| {
            acc | (self.memory[(address - MEMORY_BASE_ADDRESS) as usize + i] as u64) << (8 * i)
        })
    }

    pub fn store(&mut self, address: u64, value: u64, size: Size) {
        for i in 0..size as usize {
            self.memory[(address - MEMORY_BASE_ADDRESS) as usize + i] = (value >> (i * 8)) as u8;
        }
    }
}
