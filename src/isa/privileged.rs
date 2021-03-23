pub enum PrivilegeMode {
    MachineMode,
    SupervisorMode,
    UserMode,
}

impl Default for PrivilegeMode {
    fn default() -> Self {
        PrivilegeMode::MachineMode
    }
}
