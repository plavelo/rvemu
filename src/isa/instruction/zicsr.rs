use std::fmt;

pub enum ZicsrOpcodeR {}

#[derive(Debug, PartialEq)]
pub enum ZicsrOpcodeI {
    Csrrw,
    Csrrs,
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
}

impl fmt::Display for ZicsrOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Csrrw => f.write_str("Zicsr::Csrrw"),
            Self::Csrrs => f.write_str("Zicsr::Csrrs"),
            Self::Csrrc => f.write_str("Zicsr::Csrrc"),
            Self::Csrrwi => f.write_str("Zicsr::Csrrwi"),
            Self::Csrrsi => f.write_str("Zicsr::Csrrsi"),
            Self::Csrrci => f.write_str("Zicsr::Csrrci"),
        }
    }
}

pub enum ZicsrOpcodeS {}
pub enum ZicsrOpcodeB {}
pub enum ZicsrOpcodeU {}
pub enum ZicsrOpcodeJ {}
