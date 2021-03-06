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
            Rv64iOpcodeR::Sllw => f.write_str("Rv64i::Sllw"),
            Rv64iOpcodeR::Srlw => f.write_str("Rv64i::Srlw"),
            Rv64iOpcodeR::Sraw => f.write_str("Rv64i::Sraw"),
            Rv64iOpcodeR::Addw => f.write_str("Rv64i::Addw"),
            Rv64iOpcodeR::Subw => f.write_str("Rv64i::Subw"),
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
            Rv64iOpcodeI::Slliw => f.write_str("Rv64i::Slliw"),
            Rv64iOpcodeI::Srliw => f.write_str("Rv64i::Srliw"),
            Rv64iOpcodeI::Sraiw => f.write_str("Rv64i::Sraiw"),
            Rv64iOpcodeI::Addiw => f.write_str("Rv64i::Addiw"),
            Rv64iOpcodeI::Lwu => f.write_str("Rv64i::Lwu"),
            Rv64iOpcodeI::Ld => f.write_str("Rv64i::Ld"),
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
            Rv64iOpcodeS::Sd => f.write_str("Rv64i::Sd"),
        }
    }
}

pub enum Rv64iOpcodeB {}
pub enum Rv64iOpcodeU {}
pub enum Rv64iOpcodeJ {}
