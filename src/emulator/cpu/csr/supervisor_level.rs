use crate::{emulator::cpu::csr::Csr, isa::csr::supervisor_level::*};
use std::collections::HashMap;

pub struct SupervisorLevelCsr {
    csr: HashMap<u64, u64>,
}

impl Csr for SupervisorLevelCsr {
    fn contains(&self, address: u64) -> bool {
        self.csr.contains_key(&address)
    }

    fn get(&self, address: u64) -> u64 {
        self.csr[&address]
    }

    fn csrrw(&mut self, address: u64, value: u64) -> u64 {
        let t = self.csr[&address];
        *self.csr.get_mut(&address).unwrap() = value;
        t
    }

    fn csrrs(&mut self, address: u64, value: u64) -> u64 {
        let t = self.csr[&address];
        *self.csr.get_mut(&address).unwrap() = self.csr[&address] | value;
        t
    }

    fn csrrc(&mut self, address: u64, value: u64) -> u64 {
        let t = self.csr[&address];
        *self.csr.get_mut(&address).unwrap() = self.csr[&address] & !value;
        t
    }
}

impl Default for SupervisorLevelCsr {
    fn default() -> Self {
        Self {
            csr: [
                // Supervisor Trap Setup (SRW)
                SSTATUS, SEDELEG, SIDELEG, SIE, STVEC, SCOUNTEREN,
                // Supervisor Trap Handling (SRW)
                SSCRATCH, SEPC, SCAUSE, STVAL, SIP,
                // Supervisor Protection and Translation (SRW)
                SATP,
            ]
            .iter()
            .cloned()
            .map(|a| (a, 0))
            .collect::<HashMap<_, _>>(),
        }
    }
}
