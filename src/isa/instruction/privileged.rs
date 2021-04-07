use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PrivilegedOpcodeR {
    Uret,
    Sret,
    Mret,
    Wfi,
    SfenceVma,
}

impl fmt::Display for PrivilegedOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Uret => f.write_str("Privilege::Uret"),
            Self::Sret => f.write_str("Privilege::Sret"),
            Self::Mret => f.write_str("Privilege::Mret"),
            Self::Wfi => f.write_str("Privilege::Wfi"),
            Self::SfenceVma => f.write_str("Privilege::SfenceVma"),
        }
    }
}

pub enum PrivilegedOpcodeI {}
pub enum PrivilegedOpcodeS {}
pub enum PrivilegedOpcodeB {}
pub enum PrivilegedOpcodeU {}
pub enum PrivilegedOpcodeJ {}
