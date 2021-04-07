use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rv64mOpcodeR {
    Mulw,
    Divw,
    Divuw,
    Remw,
    Remuw,
}

impl fmt::Display for Rv64mOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Mulw => f.write_str("Rv64m::Mulw"),
            Self::Divw => f.write_str("Rv64m::Divw"),
            Self::Divuw => f.write_str("Rv64m::Divuw"),
            Self::Remw => f.write_str("Rv64m::Remw"),
            Self::Remuw => f.write_str("Rv64m::Remuw"),
        }
    }
}

pub enum Rv64mOpcodeI {}
pub enum Rv64mOpcodeS {}
pub enum Rv64mOpcodeB {}
pub enum Rv64mOpcodeU {}
pub enum Rv64mOpcodeJ {}
