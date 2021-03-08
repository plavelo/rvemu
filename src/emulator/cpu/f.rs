pub struct FloatingPointRegister {
    f: [f32; 32],
}

impl Default for FloatingPointRegister {
    fn default() -> Self {
        Self { f: [0f32; 32] }
    }
}

impl FloatingPointRegister {
    pub fn read(&self, register: usize) -> f32 {
        self.f[register]
    }

    pub fn write(&mut self, register: usize, value: f32) {
        self.f[register] = value;
    }
}
