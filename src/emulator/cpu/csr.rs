mod machine_level;
mod supervisor_level;
mod user_level;

use crate::emulator::cpu::csr::{
    machine_level::MachineLevelCsr, supervisor_level::SupervisorLevelCsr, user_level::UserLevelCsr,
};

#[derive(Default)]
pub struct ControlAndStatusRegister {
    ucsr: UserLevelCsr,
    scsr: SupervisorLevelCsr,
    mcsr: MachineLevelCsr,
}

impl ControlAndStatusRegister {
    pub fn csrrw(&mut self, address: u64, value: u64) -> u64 {
        if self.ucsr.contains(address) {
            return self.ucsr.csrrw(address, value);
        }
        if self.scsr.contains(address) {
            return self.scsr.csrrw(address, value);
        }
        if self.mcsr.contains(address) {
            return self.mcsr.csrrw(address, value);
        }
        panic!("todo: address not found.");
    }

    pub fn csrrs(&mut self, address: u64, value: u64) -> u64 {
        if self.ucsr.contains(address) {
            return self.ucsr.csrrs(address, value);
        }
        if self.scsr.contains(address) {
            return self.scsr.csrrs(address, value);
        }
        if self.mcsr.contains(address) {
            return self.mcsr.csrrs(address, value);
        }
        panic!("todo: address not found.");
    }

    pub fn csrrc(&mut self, address: u64, value: u64) -> u64 {
        if self.ucsr.contains(address) {
            return self.ucsr.csrrc(address, value);
        }
        if self.scsr.contains(address) {
            return self.scsr.csrrc(address, value);
        }
        if self.mcsr.contains(address) {
            return self.mcsr.csrrc(address, value);
        }
        panic!("todo: address not found.");
    }
}

pub trait Csr {
    fn contains(&self, address: u64) -> bool;
    fn csrrw(&mut self, address: u64, value: u64) -> u64;
    fn csrrs(&mut self, address: u64, value: u64) -> u64;
    fn csrrc(&mut self, address: u64, value: u64) -> u64;
}
