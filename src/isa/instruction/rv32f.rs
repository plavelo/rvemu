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
    FcvtWs,
    FcvtWuS,
    FmvXw,
    FeqS,
    FltS,
    FleS,
    FclassS,
    FcvtSw,
    FcvtSWu,
    FmvWx,
}

impl fmt::Display for Rv32fOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::FmaddS => f.write_str("Rv32f::FmaddS"),
            Self::FmsubS => f.write_str("Rv32f::FmsubS"),
            Self::FnmsubS => f.write_str("Rv32f::FnmsubS"),
            Self::FnmaddS => f.write_str("Rv32f::FnmaddS"),
            Self::FaddS => f.write_str("Rv32f::FaddS"),
            Self::FsubS => f.write_str("Rv32f::FsubS"),
            Self::FmulS => f.write_str("Rv32f::FmulS"),
            Self::FdivS => f.write_str("Rv32f::FdivS"),
            Self::FsqrtS => f.write_str("Rv32f::FsqrtS"),
            Self::FsgnjS => f.write_str("Rv32f::FsgnjS"),
            Self::FsgnjnS => f.write_str("Rv32f::FsgnjnS"),
            Self::FsgnjxS => f.write_str("Rv32f::FsgnjxS"),
            Self::FminS => f.write_str("Rv32f::FminS"),
            Self::FmaxS => f.write_str("Rv32f::FmaxS"),
            Self::FcvtWs => f.write_str("Rv32f::FcvtWS"),
            Self::FcvtWuS => f.write_str("Rv32f::FcvtWuS"),
            Self::FmvXw => f.write_str("Rv32f::FmvXW"),
            Self::FeqS => f.write_str("Rv32f::FeqS"),
            Self::FltS => f.write_str("Rv32f::FltS"),
            Self::FleS => f.write_str("Rv32f::FleS"),
            Self::FclassS => f.write_str("Rv32f::FclassS"),
            Self::FcvtSw => f.write_str("Rv32f::FcvtSW"),
            Self::FcvtSWu => f.write_str("Rv32f::FcvatSWu"),
            Self::FmvWx => f.write_str("Rv32f::FmvWX"),
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
