use crate::{emulator::cpu::csr::Csr, isa::csr::user_level::*};
use std::collections::HashMap;

pub struct UserLevelCsr {
    csr: HashMap<u64, u64>,
}

impl Csr for UserLevelCsr {
    fn contains(&self, address: u64) -> bool {
        self.csr.contains_key(&address)
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

impl Default for UserLevelCsr {
    fn default() -> Self {
        Self {
            csr: [
                // User Trap Setup (URW)
                USTATUS,
                UIE,
                UTVEC,
                // User Trap Handling (URW)
                USCRATCH,
                UEPC,
                UCAUSE,
                UTVAL,
                UIP,
                // User Floating-Point CSRs (URW)
                FFLAGS,
                FRM,
                FCSR,
                // User Counter/Timers (URO)
                CYCLE,
                TIME,
                INSTRET,
                HPMCOUNTER3,
                HPMCOUNTER4,
                HPMCOUNTER5,
                HPMCOUNTER6,
                HPMCOUNTER7,
                HPMCOUNTER8,
                HPMCOUNTER9,
                HPMCOUNTER10,
                HPMCOUNTER11,
                HPMCOUNTER12,
                HPMCOUNTER13,
                HPMCOUNTER14,
                HPMCOUNTER15,
                HPMCOUNTER16,
                HPMCOUNTER17,
                HPMCOUNTER18,
                HPMCOUNTER19,
                HPMCOUNTER20,
                HPMCOUNTER21,
                HPMCOUNTER22,
                HPMCOUNTER23,
                HPMCOUNTER24,
                HPMCOUNTER25,
                HPMCOUNTER26,
                HPMCOUNTER27,
                HPMCOUNTER28,
                HPMCOUNTER29,
                HPMCOUNTER30,
                HPMCOUNTER31,
                CYCLEH,
                TIMEH,
                INSTRETH,
                HPMCOUNTER3H,
                HPMCOUNTER4H,
                HPMCOUNTER5H,
                HPMCOUNTER6H,
                HPMCOUNTER7H,
                HPMCOUNTER8H,
                HPMCOUNTER9H,
                HPMCOUNTER10H,
                HPMCOUNTER11H,
                HPMCOUNTER12H,
                HPMCOUNTER13H,
                HPMCOUNTER14H,
                HPMCOUNTER15H,
                HPMCOUNTER16H,
                HPMCOUNTER17H,
                HPMCOUNTER18H,
                HPMCOUNTER19H,
                HPMCOUNTER20H,
                HPMCOUNTER21H,
                HPMCOUNTER22H,
                HPMCOUNTER23H,
                HPMCOUNTER24H,
                HPMCOUNTER25H,
                HPMCOUNTER26H,
                HPMCOUNTER27H,
                HPMCOUNTER28H,
                HPMCOUNTER29H,
                HPMCOUNTER30H,
                HPMCOUNTER31H,
            ]
            .iter()
            .cloned()
            .map(|a| (a, 0))
            .collect::<HashMap<_, _>>(),
        }
    }
}
