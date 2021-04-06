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
