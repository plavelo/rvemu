use std::fmt;

pub enum ZifenceiOpcodeR {}

#[derive(Debug, PartialEq)]
pub enum ZifenceiOpcodeI {
    FenceI,
}

impl fmt::Display for ZifenceiOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::FenceI => f.write_str("Zifencei::FenceI"),
        }
    }
}

pub enum ZifenceiOpcodeS {}
pub enum ZifenceiOpcodeB {}
pub enum ZifenceiOpcodeU {}
pub enum ZifenceiOpcodeJ {}
