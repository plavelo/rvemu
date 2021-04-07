use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rv64iOpcodeR {
    Sllw,
    Srlw,
    Sraw,
    Addw,
    Subw,
}

impl fmt::Display for Rv64iOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Sllw => f.write_str("Rv64i::Sllw"),
            Self::Srlw => f.write_str("Rv64i::Srlw"),
            Self::Sraw => f.write_str("Rv64i::Sraw"),
            Self::Addw => f.write_str("Rv64i::Addw"),
            Self::Subw => f.write_str("Rv64i::Subw"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv64iOpcodeI {
    Slliw,
    Srliw,
    Sraiw,
    Addiw,
    Lwu,
    Ld,
}

impl fmt::Display for Rv64iOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Slliw => f.write_str("Rv64i::Slliw"),
            Self::Srliw => f.write_str("Rv64i::Srliw"),
            Self::Sraiw => f.write_str("Rv64i::Sraiw"),
            Self::Addiw => f.write_str("Rv64i::Addiw"),
            Self::Lwu => f.write_str("Rv64i::Lwu"),
            Self::Ld => f.write_str("Rv64i::Ld"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv64iOpcodeS {
    Sd,
}

impl fmt::Display for Rv64iOpcodeS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Sd => f.write_str("Rv64i::Sd"),
        }
    }
}

pub enum Rv64iOpcodeB {}
pub enum Rv64iOpcodeU {}
pub enum Rv64iOpcodeJ {}
