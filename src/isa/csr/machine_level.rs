// Machine Information Registers (MRO)
pub const MVENDORID: u64 = 0xf11; // Vendor ID.
pub const MARCHID: u64 = 0xf12; // Architecture ID.
pub const MIMPID: u64 = 0xf13; // Implementation ID.
pub const MHARTID: u64 = 0xf14; // Hardware thread ID.

// Machine Trap Setup (MRW)
pub const MSTATUS: u64 = 0x300; // Machine status register.
pub const MISA: u64 = 0x301; // ISA and extensions.
pub const MEDELEG: u64 = 0x302; // Machine exception delegation register.
pub const MIDELEG: u64 = 0x303; // Machine interrupt delegation register.
pub const MIE: u64 = 0x304; // Machine interrupt-enable register.
pub const MTVEC: u64 = 0x305; // Machine trap-handler base address.
pub const MCOUNTEREN: u64 = 0x306; // Machine counter enable.

// Machine Trap Handling (MRW)
pub const MSCRATCH: u64 = 0x340; // Scratch register for machine trap handlers.
pub const MEPC: u64 = 0x341; // Machine exception program counter.
pub const MCAUSE: u64 = 0x342; // Machine trap cause.
pub const MTVAL: u64 = 0x343; // Machine bad address or instruction.
pub const MIP: u64 = 0x344; // Machine interrupt pending.

// Machine Memory Protection (MRW)
pub const PMPCFG0: u64 = 0x3a0; // Physical memory protection configuration.
pub const PMPCFG1: u64 = 0x3a1; // Physical memory protection configuration, RV32 only.
pub const PMPCFG2: u64 = 0x3a2; // Physical memory protection configuration.
pub const PMPCFG3: u64 = 0x3a3; // Physical memory protection configuration, RV32 only.
pub const PMPADDR0: u64 = 0x3b0; // Physical memory protection address register.
pub const PMPADDR1: u64 = 0x3b1; // Physical memory protection address register.
pub const PMPADDR2: u64 = 0x3b2; // Physical memory protection address register.
pub const PMPADDR3: u64 = 0x3b3; // Physical memory protection address register.
pub const PMPADDR4: u64 = 0x3b4; // Physical memory protection address register.
pub const PMPADDR5: u64 = 0x3b5; // Physical memory protection address register.
pub const PMPADDR6: u64 = 0x3b6; // Physical memory protection address register.
pub const PMPADDR7: u64 = 0x3b7; // Physical memory protection address register.
pub const PMPADDR8: u64 = 0x3b8; // Physical memory protection address register.
pub const PMPADDR9: u64 = 0x3b9; // Physical memory protection address register.
pub const PMPADDR10: u64 = 0x3ba; // Physical memory protection address register.
pub const PMPADDR11: u64 = 0x3bb; // Physical memory protection address register.
pub const PMPADDR12: u64 = 0x3bc; // Physical memory protection address register.
pub const PMPADDR13: u64 = 0x3bd; // Physical memory protection address register.
pub const PMPADDR14: u64 = 0x3be; // Physical memory protection address register.
pub const PMPADDR15: u64 = 0x3bf; // Physical memory protection address register.

// Machine Counter/Timers (MRW)
pub const MCYCLE: u64 = 0xb00; // Machine cycle counter.
pub const MINSTRET: u64 = 0xb02; // Machine instructions-retired counter.
pub const MHPMCOUNTER: u64 = 0xb03; // Machine performance-monitoring counter.
pub const MHPMCOUNTER4: u64 = 0xb04; // Machine performance-monitoring counter.
pub const MHPMCOUNTER5: u64 = 0xb05; // Machine performance-monitoring counter.
pub const MHPMCOUNTER6: u64 = 0xb06; // Machine performance-monitoring counter.
pub const MHPMCOUNTER7: u64 = 0xb07; // Machine performance-monitoring counter.
pub const MHPMCOUNTER8: u64 = 0xb08; // Machine performance-monitoring counter.
pub const MHPMCOUNTER9: u64 = 0xb09; // Machine performance-monitoring counter.
pub const MHPMCOUNTER10: u64 = 0xb0a; // Machine performance-monitoring counter.
pub const MHPMCOUNTER11: u64 = 0xb0b; // Machine performance-monitoring counter.
pub const MHPMCOUNTER12: u64 = 0xb0c; // Machine performance-monitoring counter.
pub const MHPMCOUNTER13: u64 = 0xb0d; // Machine performance-monitoring counter.
pub const MHPMCOUNTER14: u64 = 0xb0e; // Machine performance-monitoring counter.
pub const MHPMCOUNTER15: u64 = 0xb0f; // Machine performance-monitoring counter.
pub const MHPMCOUNTER16: u64 = 0xb10; // Machine performance-monitoring counter.
pub const MHPMCOUNTER17: u64 = 0xb11; // Machine performance-monitoring counter.
pub const MHPMCOUNTER18: u64 = 0xb12; // Machine performance-monitoring counter.
pub const MHPMCOUNTER19: u64 = 0xb13; // Machine performance-monitoring counter.
pub const MHPMCOUNTER20: u64 = 0xb14; // Machine performance-monitoring counter.
pub const MHPMCOUNTER21: u64 = 0xb15; // Machine performance-monitoring counter.
pub const MHPMCOUNTER22: u64 = 0xb16; // Machine performance-monitoring counter.
pub const MHPMCOUNTER23: u64 = 0xb17; // Machine performance-monitoring counter.
pub const MHPMCOUNTER24: u64 = 0xb18; // Machine performance-monitoring counter.
pub const MHPMCOUNTER25: u64 = 0xb19; // Machine performance-monitoring counter.
pub const MHPMCOUNTER26: u64 = 0xb1a; // Machine performance-monitoring counter.
pub const MHPMCOUNTER27: u64 = 0xb1b; // Machine performance-monitoring counter.
pub const MHPMCOUNTER28: u64 = 0xb1c; // Machine performance-monitoring counter.
pub const MHPMCOUNTER29: u64 = 0xb1d; // Machine performance-monitoring counter.
pub const MHPMCOUNTER30: u64 = 0xb1e; // Machine performance-monitoring counter.
pub const MHPMCOUNTER31: u64 = 0xb1f; // Machine performance-monitoring counter.
pub const MCYCLEH: u64 = 0xb80; // Upper 32 bits of mcycle, RV32I only.
pub const MINSTRETH: u64 = 0xb82; // Upper 32 bits of minstret, RV32I only.
pub const MHPMCOUNTER3H: u64 = 0xb83; // Upper 32 bits of mhpmcounter3, RV32I only.
pub const MHPMCOUNTER31H: u64 = 0xb9f; // Upper 32 bits of mhpmcounter31, RV32I only.

// Machine Counter Setup (MRW)
pub const MCOUNTINHIBIT: u64 = 0x320; // Machine counter-inhibit register.
pub const MHPMEVENT3: u64 = 0x323; // Machine performance-monitoring event selector.
pub const MHPMEVENT4: u64 = 0x324; // Machine performance-monitoring event selector.
pub const MHPMEVENT5: u64 = 0x325; // Machine performance-monitoring event selector.
pub const MHPMEVENT6: u64 = 0x326; // Machine performance-monitoring event selector.
pub const MHPMEVENT7: u64 = 0x327; // Machine performance-monitoring event selector.
pub const MHPMEVENT8: u64 = 0x328; // Machine performance-monitoring event selector.
pub const MHPMEVENT9: u64 = 0x329; // Machine performance-monitoring event selector.
pub const MHPMEVENT10: u64 = 0x32a; // Machine performance-monitoring event selector.
pub const MHPMEVENT11: u64 = 0x32b; // Machine performance-monitoring event selector.
pub const MHPMEVENT12: u64 = 0x32c; // Machine performance-monitoring event selector.
pub const MHPMEVENT13: u64 = 0x32d; // Machine performance-monitoring event selector.
pub const MHPMEVENT14: u64 = 0x32e; // Machine performance-monitoring event selector.
pub const MHPMEVENT15: u64 = 0x32f; // Machine performance-monitoring event selector.
pub const MHPMEVENT16: u64 = 0x330; // Machine performance-monitoring event selector.
pub const MHPMEVENT17: u64 = 0x331; // Machine performance-monitoring event selector.
pub const MHPMEVENT18: u64 = 0x332; // Machine performance-monitoring event selector.
pub const MHPMEVENT19: u64 = 0x333; // Machine performance-monitoring event selector.
pub const MHPMEVENT20: u64 = 0x334; // Machine performance-monitoring event selector.
pub const MHPMEVENT21: u64 = 0x335; // Machine performance-monitoring event selector.
pub const MHPMEVENT22: u64 = 0x336; // Machine performance-monitoring event selector.
pub const MHPMEVENT23: u64 = 0x337; // Machine performance-monitoring event selector.
pub const MHPMEVENT24: u64 = 0x338; // Machine performance-monitoring event selector.
pub const MHPMEVENT25: u64 = 0x339; // Machine performance-monitoring event selector.
pub const MHPMEVENT26: u64 = 0x33a; // Machine performance-monitoring event selector.
pub const MHPMEVENT27: u64 = 0x33b; // Machine performance-monitoring event selector.
pub const MHPMEVENT28: u64 = 0x33c; // Machine performance-monitoring event selector.
pub const MHPMEVENT29: u64 = 0x33d; // Machine performance-monitoring event selector.
pub const MHPMEVENT30: u64 = 0x33e; // Machine performance-monitoring event selector.
pub const MHPMEVENT31: u64 = 0x33f; // Machine performance-monitoring event selector.

// Debug/Trace Registers (shared with Debug Mode) (MRW)
pub const TSELECT: u64 = 0x7a0; // Debug/Trace trigger register select.
pub const TDATA1: u64 = 0x7a1; // First Debug/Trace trigger data register.
pub const TDATA2: u64 = 0x7a2; // Second Debug/Trace trigger data register.
pub const TDATA3: u64 = 0x7a3; // Third Debug/Trace trigger data register.

// Debug Mode Registers (DRW)
pub const DCSR: u64 = 0x7b0; // Debug control and status register.
pub const DPC: u64 = 0x7b1; // Debug PC.
pub const DSCRATCH0: u64 = 0x7b2; // Debug scratch register 0.
pub const DSCRATCH1: u64 = 0x7b3; // Debug scratch register 1.
