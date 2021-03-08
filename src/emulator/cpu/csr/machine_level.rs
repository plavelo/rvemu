use crate::{emulator::cpu::csr::Csr, isa::csr::machine_level::*};
use std::collections::HashMap;

pub struct MachineLevelCsr {
    csr: HashMap<u64, u64>,
}

impl Csr for MachineLevelCsr {
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

impl Default for MachineLevelCsr {
    fn default() -> Self {
        Self {
            csr: [
                // Machine Information Registers (MRO)
                MVENDORID,
                MARCHID,
                MIMPID,
                MHARTID,
                // Machine Trap Setup (MRW)
                MSTATUS,
                MISA,
                MEDELEG,
                MIDELEG,
                MIE,
                MTVEC,
                MCOUNTEREN,
                // Machine Trap Handling (MRW)
                MSCRATCH,
                MEPC,
                MCAUSE,
                MTVAL,
                MIP,
                // Machine Memory Protection (MRW)
                PMPCFG0,
                PMPCFG1,
                PMPCFG2,
                PMPCFG3,
                PMPADDR0,
                PMPADDR1,
                PMPADDR2,
                PMPADDR3,
                PMPADDR4,
                PMPADDR5,
                PMPADDR6,
                PMPADDR7,
                PMPADDR8,
                PMPADDR9,
                PMPADDR10,
                PMPADDR11,
                PMPADDR12,
                PMPADDR13,
                PMPADDR14,
                PMPADDR15,
                // Machine Counter/Timers (MRW)
                MCYCLE,
                MINSTRET,
                MHPMCOUNTER,
                MHPMCOUNTER4,
                MHPMCOUNTER5,
                MHPMCOUNTER6,
                MHPMCOUNTER7,
                MHPMCOUNTER8,
                MHPMCOUNTER9,
                MHPMCOUNTER10,
                MHPMCOUNTER11,
                MHPMCOUNTER12,
                MHPMCOUNTER13,
                MHPMCOUNTER14,
                MHPMCOUNTER15,
                MHPMCOUNTER16,
                MHPMCOUNTER17,
                MHPMCOUNTER18,
                MHPMCOUNTER19,
                MHPMCOUNTER20,
                MHPMCOUNTER21,
                MHPMCOUNTER22,
                MHPMCOUNTER23,
                MHPMCOUNTER24,
                MHPMCOUNTER25,
                MHPMCOUNTER26,
                MHPMCOUNTER27,
                MHPMCOUNTER28,
                MHPMCOUNTER29,
                MHPMCOUNTER30,
                MHPMCOUNTER31,
                MCYCLEH,
                MINSTRETH,
                MHPMCOUNTER3H,
                MHPMCOUNTER31H,
                // Machine Counter Setup (MRW)
                MCOUNTINHIBIT,
                MHPMEVENT3,
                MHPMEVENT4,
                MHPMEVENT5,
                MHPMEVENT6,
                MHPMEVENT7,
                MHPMEVENT8,
                MHPMEVENT9,
                MHPMEVENT10,
                MHPMEVENT11,
                MHPMEVENT12,
                MHPMEVENT13,
                MHPMEVENT14,
                MHPMEVENT15,
                MHPMEVENT16,
                MHPMEVENT17,
                MHPMEVENT18,
                MHPMEVENT19,
                MHPMEVENT20,
                MHPMEVENT21,
                MHPMEVENT22,
                MHPMEVENT23,
                MHPMEVENT24,
                MHPMEVENT25,
                MHPMEVENT26,
                MHPMEVENT27,
                MHPMEVENT28,
                MHPMEVENT29,
                MHPMEVENT30,
                MHPMEVENT31,
                // Debug/Trace Registers (shared with Debug Mode) (MRW)
                TSELECT,
                TDATA1,
                TDATA2,
                TDATA3,
                // Debug Mode Registers (DRW)
                DCSR,
                DPC,
                DSCRATCH0,
                DSCRATCH1,
            ]
            .iter()
            .cloned()
            .map(|a| (a, 0))
            .collect::<HashMap<_, _>>(),
        }
    }
}
