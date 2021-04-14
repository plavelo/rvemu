use crate::emulator::bus::memory::MEMORY_SIZE;

pub const ZERO: usize = 0;
#[allow(dead_code)]
pub const RA: usize = 1;
pub const SP: usize = 2;
#[allow(dead_code)]
pub const GP: usize = 3;
#[allow(dead_code)]
pub const TP: usize = 4;
#[allow(dead_code)]
pub const T0: usize = 5;
#[allow(dead_code)]
pub const T1: usize = 6;
#[allow(dead_code)]
pub const T2: usize = 7;
#[allow(dead_code)]
pub const S0: usize = 8;
#[allow(dead_code)]
pub const FP: usize = 8;
#[allow(dead_code)]
pub const S1: usize = 9;
#[allow(dead_code)]
pub const A0: usize = 10;
#[allow(dead_code)]
pub const A1: usize = 11;
#[allow(dead_code)]
pub const A2: usize = 12;
#[allow(dead_code)]
pub const A3: usize = 13;
#[allow(dead_code)]
pub const A4: usize = 14;
#[allow(dead_code)]
pub const A5: usize = 15;
#[allow(dead_code)]
pub const A6: usize = 16;
#[allow(dead_code)]
pub const A7: usize = 17;
#[allow(dead_code)]
pub const S2: usize = 18;
#[allow(dead_code)]
pub const S3: usize = 19;
#[allow(dead_code)]
pub const S4: usize = 20;
#[allow(dead_code)]
pub const S5: usize = 21;
#[allow(dead_code)]
pub const S6: usize = 22;
#[allow(dead_code)]
pub const S7: usize = 23;
#[allow(dead_code)]
pub const S8: usize = 24;
#[allow(dead_code)]
pub const S9: usize = 25;
#[allow(dead_code)]
pub const S10: usize = 26;
#[allow(dead_code)]
pub const S11: usize = 27;
#[allow(dead_code)]
pub const T3: usize = 28;
#[allow(dead_code)]
pub const T4: usize = 29;
#[allow(dead_code)]
pub const T5: usize = 30;
#[allow(dead_code)]
pub const T6: usize = 31;

pub struct IntegerRegister {
    x: [u64; 32],
}

impl Default for IntegerRegister {
    fn default() -> Self {
        let mut x = [0; 32];
        x[SP] = MEMORY_SIZE;
        Self { x }
    }
}

impl IntegerRegister {
    pub fn readi(&self, register: usize) -> i64 {
        self.x[register] as i64
    }

    pub fn readu(&self, register: usize) -> u64 {
        self.x[register]
    }

    pub fn writei(&mut self, register: usize, value: i64) {
        if register != ZERO {
            self.x[register] = value as u64;
        }
    }

    pub fn writeu(&mut self, register: usize, value: u64) {
        if register != ZERO {
            self.x[register] = value;
        }
    }
}
