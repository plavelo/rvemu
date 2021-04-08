#[derive(Copy, Clone, PartialEq)]
pub enum PrivilegeMode {
    UserMode = 0b00,
    SupervisorMode = 0b01,
    MachineMode = 0b11,
}

impl Default for PrivilegeMode {
    fn default() -> Self {
        PrivilegeMode::MachineMode
    }
}

impl PrivilegeMode {
    pub fn from_primitive(mode: u64) -> Self {
        match mode {
            0 => Self::UserMode,
            0b01 => Self::SupervisorMode,
            0b11 => Self::MachineMode,
            _ => panic!(),
        }
    }
}
