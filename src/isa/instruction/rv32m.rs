use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rv32mOpcodeR {
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

impl fmt::Display for Rv32mOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Mul => f.write_str("Rv32m::Mul"),
            Self::Mulh => f.write_str("Rv32m::Mulh"),
            Self::Mulhsu => f.write_str("Rv32m::Mulhsu"),
            Self::Mulhu => f.write_str("Rv32m::Mulhu"),
            Self::Div => f.write_str("Rv32m::Div"),
            Self::Divu => f.write_str("Rv32m::Divu"),
            Self::Rem => f.write_str("Rv32m::Rem"),
            Self::Remu => f.write_str("Rv32m::Remu"),
        }
    }
}

pub enum Rv32mOpcodeI {}
pub enum Rv32mOpcodeS {}
pub enum Rv32mOpcodeB {}
pub enum Rv32mOpcodeU {}
pub enum Rv32mOpcodeJ {}
