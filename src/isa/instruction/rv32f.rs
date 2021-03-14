use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rv32fOpcodeR {
    FmaddS,
    FmsubS,
    FnmsubS,
    FnmaddS,
    FaddS,
    FsubS,
    FmulS,
    FdivS,
    FsqrtS,
    FsgnjS,
    FsgnjnS,
    FsgnjxS,
    FminS,
    FmaxS,
    FcvtWS,
    FcvtWuS,
    FmvXW,
    FeqS,
    FltS,
    FleS,
    FclassS,
    FcvtSW,
    FcvtSWu,
    FmvWX,
}

impl fmt::Display for Rv32fOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32fOpcodeR::FmaddS => f.write_str("Rv32f::FmaddS"),
            Rv32fOpcodeR::FmsubS => f.write_str("Rv32f::FmsubS"),
            Rv32fOpcodeR::FnmsubS => f.write_str("Rv32f::FnmsubS"),
            Rv32fOpcodeR::FnmaddS => f.write_str("Rv32f::FnmaddS"),
            Rv32fOpcodeR::FaddS => f.write_str("Rv32f::FaddS"),
            Rv32fOpcodeR::FsubS => f.write_str("Rv32f::FsubS"),
            Rv32fOpcodeR::FmulS => f.write_str("Rv32f::FmulS"),
            Rv32fOpcodeR::FdivS => f.write_str("Rv32f::FdivS"),
            Rv32fOpcodeR::FsqrtS => f.write_str("Rv32f::FsqrtS"),
            Rv32fOpcodeR::FsgnjS => f.write_str("Rv32f::FsgnjS"),
            Rv32fOpcodeR::FsgnjnS => f.write_str("Rv32f::FsgnjnS"),
            Rv32fOpcodeR::FsgnjxS => f.write_str("Rv32f::FsgnjxS"),
            Rv32fOpcodeR::FminS => f.write_str("Rv32f::FminS"),
            Rv32fOpcodeR::FmaxS => f.write_str("Rv32f::FmaxS"),
            Rv32fOpcodeR::FcvtWS => f.write_str("Rv32f::FcvtWS"),
            Rv32fOpcodeR::FcvtWuS => f.write_str("Rv32f::FcvtWuS"),
            Rv32fOpcodeR::FmvXW => f.write_str("Rv32f::FmvXW"),
            Rv32fOpcodeR::FeqS => f.write_str("Rv32f::FeqS"),
            Rv32fOpcodeR::FltS => f.write_str("Rv32f::FltS"),
            Rv32fOpcodeR::FleS => f.write_str("Rv32f::FleS"),
            Rv32fOpcodeR::FclassS => f.write_str("Rv32f::FclassS"),
            Rv32fOpcodeR::FcvtSW => f.write_str("Rv32f::FcvtSW"),
            Rv32fOpcodeR::FcvtSWu => f.write_str("Rv32f::FcvatSWu"),
            Rv32fOpcodeR::FmvWX => f.write_str("Rv32f::FmvWX"),
        }
    }
}

pub enum Rv32fOpcodeI {
    Flw,
}
pub enum Rv32fOpcodeS {
    Fsw,
}
pub enum Rv32fOpcodeB {}
pub enum Rv32fOpcodeU {}
pub enum Rv32fOpcodeJ {}
