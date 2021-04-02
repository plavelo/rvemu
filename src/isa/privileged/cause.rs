pub enum Cause {
    UserSoftwareInterrupt,
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    UserTimerInterrupt,
    SupervisorTimerInterrupt,
    MachineTimerInterrupt,
    UserExternalInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt,
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUmode,
    EnvironmentCallFromSmode,
    EnvironmentCallFromMmode,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
}

impl Cause {
    pub fn to_primitive(&self) -> u64 {
        match self {
            Cause::UserSoftwareInterrupt => 1u64 << 63 | 0,
            Cause::SupervisorSoftwareInterrupt => 1u64 << 63 | 1,
            Cause::MachineSoftwareInterrupt => 1u64 << 63 | 3,
            Cause::UserTimerInterrupt => 1u64 << 63 | 4,
            Cause::SupervisorTimerInterrupt => 1u64 << 63 | 5,
            Cause::MachineTimerInterrupt => 1u64 << 63 | 7,
            Cause::UserExternalInterrupt => 1u64 << 63 | 8,
            Cause::SupervisorExternalInterrupt => 1u64 << 63 | 9,
            Cause::MachineExternalInterrupt => 1u64 << 63 | 11,
            Cause::InstructionAddressMisaligned => 0,
            Cause::InstructionAccessFault => 1,
            Cause::IllegalInstruction => 2,
            Cause::Breakpoint => 3,
            Cause::LoadAddressMisaligned => 4,
            Cause::LoadAccessFault => 5,
            Cause::StoreAddressMisaligned => 6,
            Cause::StoreAccessFault => 7,
            Cause::EnvironmentCallFromUmode => 8,
            Cause::EnvironmentCallFromSmode => 9,
            Cause::EnvironmentCallFromMmode => 11,
            Cause::InstructionPageFault => 12,
            Cause::LoadPageFault => 13,
            Cause::StorePageFault => 15,
        }
    }

    pub fn is_interrupt(&self) -> bool {
        self.to_primitive() >> 63 == 1
    }

    pub fn exception_code(&self) -> u64 {
        self.to_primitive() & 0b1111
    }
}
