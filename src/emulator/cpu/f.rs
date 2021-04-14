#[allow(dead_code)]
pub struct FloatingPointRegister {
    f: [u32; 32],
}

impl Default for FloatingPointRegister {
    fn default() -> Self {
        Self { f: [0; 32] }
    }
}

#[allow(dead_code)]
impl FloatingPointRegister {
    pub fn read(&self, register: usize) -> u32 {
        self.f[register]
    }

    pub fn write(&mut self, register: usize, value: u32) {
        self.f[register] = value;
    }
}
