use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister,
            executor::{Executor, MASK_3BIT},
            f::FloatingPointRegister,
            pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::{
        csr::user_level::FCSR,
        instruction::{
            rv32f::{
                RoundingMode, Rv32fOpcodeB, Rv32fOpcodeI, Rv32fOpcodeJ, Rv32fOpcodeR, Rv32fOpcodeS,
                Rv32fOpcodeU,
            },
            Instruction,
        },
    },
};

pub struct Rv32fExecutor;

impl Executor for Rv32fExecutor {
    type OpcodeR = Rv32fOpcodeR;
    type OpcodeI = Rv32fOpcodeI;
    type OpcodeS = Rv32fOpcodeS;
    type OpcodeB = Rv32fOpcodeB;
    type OpcodeU = Rv32fOpcodeU;
    type OpcodeJ = Rv32fOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv32fOpcodeR,
            Rv32fOpcodeI,
            Rv32fOpcodeS,
            Rv32fOpcodeB,
            Rv32fOpcodeU,
            Rv32fOpcodeJ,
        >,
        _: &mut ProgramCounter,
        _: &mut IntegerRegister,
        _: &mut FloatingPointRegister,
        csr: &mut ControlAndStatusRegister,
        _: &mut SystemBus,
    ) {
        let raw_frm = (csr.csrrs(FCSR, 0) >> 5) & MASK_3BIT;
        let _frm = decode_frm(raw_frm);
        match instruction {
            Instruction::TypeR {
                opcode,
                rd: _,
                funct3: _,
                rs1: _,
                rs2: _,
                funct7: _,
            } => match opcode {
                Rv32fOpcodeR::FmaddS => {}
                Rv32fOpcodeR::FmsubS => {}
                Rv32fOpcodeR::FnmsubS => {}
                Rv32fOpcodeR::FnmaddS => {}
                Rv32fOpcodeR::FaddS => {}
                Rv32fOpcodeR::FsubS => {}
                Rv32fOpcodeR::FmulS => {}
                Rv32fOpcodeR::FdivS => {}
                Rv32fOpcodeR::FsqrtS => {}
                Rv32fOpcodeR::FsgnjS => {}
                Rv32fOpcodeR::FsgnjnS => {}
                Rv32fOpcodeR::FsgnjxS => {}
                Rv32fOpcodeR::FminS => {}
                Rv32fOpcodeR::FmaxS => {}
                Rv32fOpcodeR::FcvtWS => {}
                Rv32fOpcodeR::FcvtWuS => {}
                Rv32fOpcodeR::FmvXW => {}
                Rv32fOpcodeR::FeqS => {}
                Rv32fOpcodeR::FltS => {}
                Rv32fOpcodeR::FleS => {}
                Rv32fOpcodeR::FclassS => {}
                Rv32fOpcodeR::FcvtSW => {}
                Rv32fOpcodeR::FcvtSWu => {}
                Rv32fOpcodeR::FmvWX => {}
            },
            Instruction::TypeI {
                opcode,
                rd: _,
                funct3: _,
                rs1: _,
                imm: _,
            } => match opcode {
                Rv32fOpcodeI::Flw => {}
            },
            Instruction::TypeS {
                opcode,
                funct3: _,
                rs1: _,
                rs2: _,
                imm: _,
            } => match opcode {
                Rv32fOpcodeS::Fsw => {}
            },
            _ => {}
        }
    }
}

fn decode_frm(frm: u64) -> Option<RoundingMode> {
    match frm {
        0b000 => Some(RoundingMode::RNE),
        0b001 => Some(RoundingMode::RTZ),
        0b010 => Some(RoundingMode::RDN),
        0b011 => Some(RoundingMode::RUP),
        0b100 => Some(RoundingMode::RMM),
        0b111 => Some(RoundingMode::DYN),
        _ => None,
    }
}
