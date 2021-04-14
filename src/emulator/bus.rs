pub mod memory;
use crate::emulator::bus::memory::Memory;

pub enum Size {
    Byte = 1,
    Halfword = 2,
    Word = 4,
    Doubleword = 8,
}

#[derive(Default)]
pub struct SystemBus {
    pub memory: Memory,
}

impl SystemBus {
    pub fn load(&self, address: u64, size: Size) -> u64 {
        self.memory.load(address, size)
    }

    pub fn load8(&self, address: u64) -> u8 {
        self.load(address, Size::Byte) as u8
    }

    pub fn load16(&self, address: u64) -> u16 {
        self.load(address, Size::Halfword) as u16
    }

    pub fn load32(&self, address: u64) -> u32 {
        self.load(address, Size::Word) as u32
    }

    pub fn load64(&self, address: u64) -> u64 {
        self.load(address, Size::Doubleword)
    }

    pub fn store(&mut self, address: u64, value: u64, size: Size) {
        self.memory.store(address, value, size);
    }

    pub fn store8(&mut self, address: u64, value: u8) {
        self.store(address, value as u64, Size::Byte)
    }

    pub fn store16(&mut self, address: u64, value: u16) {
        self.store(address, value as u64, Size::Halfword)
    }

    pub fn store32(&mut self, address: u64, value: u32) {
        self.store(address, value as u64, Size::Word)
    }

    pub fn store64(&mut self, address: u64, value: u64) {
        self.store(address, value, Size::Doubleword)
    }
}
