// Supervisor Trap Setup (SRW)
pub const SSTATUS: u64 = 0x100; // Supervisor status register.
pub const SEDELEG: u64 = 0x102; // Supervisor exception delegation register.
pub const SIDELEG: u64 = 0x103; // Supervisor interrupt delegation register.
pub const SIE: u64 = 0x104; // Supervisor interrupt-enable register.
pub const STVEC: u64 = 0x105; // Supervisor trap handler base address.
pub const SCOUNTEREN: u64 = 0x106; // Supervisor counter enable.

// Supervisor Trap Handling (SRW)
pub const SSCRATCH: u64 = 0x140; // Scratch register for supervisor trap handlers.
pub const SEPC: u64 = 0x141; // Supervisor exception program counter.
pub const SCAUSE: u64 = 0x142; // Supervisor trap cause.
pub const STVAL: u64 = 0x143; // Supervisor bad address or instruction.
pub const SIP: u64 = 0x144; // Supervisor interrupt pending.

// Supervisor Protection and Translation (SRW)
pub const SATP: u64 = 0x180; // Supervisor address translation and protection.
