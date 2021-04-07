#[derive(Copy, Clone)]
pub enum PrivilegeMode {
    MachineMode = 0b11,
    SupervisorMode = 0b01,
    UserMode = 0b00,
}

impl Default for PrivilegeMode {
    fn default() -> Self {
        PrivilegeMode::MachineMode
    }
}

impl PrivilegeMode {
    pub fn from_primitive(mode: u64) -> Self {
        match mode {
            0b11 => Self::MachineMode,
            0b01 => Self::SupervisorMode,
            _ => Self::UserMode,
        }
    }
}
