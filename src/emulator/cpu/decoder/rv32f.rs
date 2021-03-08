use crate::{
    emulator::cpu::decoder::{Decoder, MASK_3BIT, MASK_5BIT, MASK_7BIT},
    isa::instruction::{
        rv32f::{
            Rv32fOpcodeB, Rv32fOpcodeI, Rv32fOpcodeJ, Rv32fOpcodeR, Rv32fOpcodeS, Rv32fOpcodeU,
        },
        Instruction,
    },
};

pub struct Rv32fDecoder;

impl Decoder for Rv32fDecoder {
    type OpcodeR = Rv32fOpcodeR;
    type OpcodeI = Rv32fOpcodeI;
    type OpcodeS = Rv32fOpcodeS;
    type OpcodeB = Rv32fOpcodeB;
    type OpcodeU = Rv32fOpcodeU;
    type OpcodeJ = Rv32fOpcodeJ;

    #[allow(clippy::type_complexity)]
    fn decode(
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        let opcode = instruction & MASK_7BIT;
        let funct3 = (instruction >> 12) & MASK_3BIT;
        let rs2 = (instruction >> 20) & MASK_5BIT;
        let funct7 = (instruction >> 25) & MASK_7BIT;
        match opcode {
            0b0000111 => Self::decode_i(
                match funct3 {
                    0b010 => Some(Rv32fOpcodeI::Flw),
                    _ => None,
                },
                instruction,
            ),
            0b0100111 => Self::decode_s(
                match funct3 {
                    0b010 => Some(Rv32fOpcodeS::Fsw),
                    _ => None,
                },
                instruction,
            ),
            0b1000011 => Self::decode_r(Some(Rv32fOpcodeR::FmaddS), instruction),
            0b1000111 => Self::decode_r(Some(Rv32fOpcodeR::FmsubS), instruction),
            0b1001011 => Self::decode_r(Some(Rv32fOpcodeR::FnmsubS), instruction),
            0b1001111 => Self::decode_r(Some(Rv32fOpcodeR::FnmaddS), instruction),
            0b1010011 => Self::decode_r(
                match funct7 {
                    0b0000000 => Some(Rv32fOpcodeR::FaddS),
                    0b0000100 => Some(Rv32fOpcodeR::FsubS),
                    0b0001000 => Some(Rv32fOpcodeR::FmulS),
                    0b0001100 => Some(Rv32fOpcodeR::FdivS),
                    0b0101100 => Some(Rv32fOpcodeR::FsqrtS),
                    0b0010000 => match funct3 {
                        0b000 => Some(Rv32fOpcodeR::FsgnjS),
                        0b001 => Some(Rv32fOpcodeR::FsgnjnS),
                        0b010 => Some(Rv32fOpcodeR::FsgnjxS),
                        _ => None,
                    },
                    0b0010100 => match funct3 {
                        0b000 => Some(Rv32fOpcodeR::FminS),
                        0b001 => Some(Rv32fOpcodeR::FmaxS),
                        _ => None,
                    },
                    0b1100000 => match rs2 {
                        0b00000 => Some(Rv32fOpcodeR::FcvtWS),
                        0b00001 => Some(Rv32fOpcodeR::FcvtWuS),
                        _ => None,
                    },
                    0b1110000 => match funct3 {
                        0b000 => Some(Rv32fOpcodeR::FmvXW),
                        0b001 => Some(Rv32fOpcodeR::FclassS),
                        _ => None,
                    },
                    0b1010000 => match funct3 {
                        0b010 => Some(Rv32fOpcodeR::FeqS),
                        0b001 => Some(Rv32fOpcodeR::FltS),
                        0b000 => Some(Rv32fOpcodeR::FleS),
                        _ => None,
                    },
                    0b1101000 => match rs2 {
                        0b00000 => Some(Rv32fOpcodeR::FcvtSW),
                        0b00001 => Some(Rv32fOpcodeR::FcvtSWu),
                        _ => None,
                    },
                    0b1111000 => Some(Rv32fOpcodeR::FmvWX),
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }
}
