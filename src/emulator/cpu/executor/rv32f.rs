use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister,
            executor::{Executor, MASK_3BIT, MASK_5BIT},
            f::FloatingPointRegister,
            pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::{
        csr::user_level::FCSR,
        instruction::{
            rv32f::{
                Rv32fOpcodeB, Rv32fOpcodeI, Rv32fOpcodeJ, Rv32fOpcodeR, Rv32fOpcodeS, Rv32fOpcodeU,
            },
            Instruction,
        },
    },
};
use softfloat_wrapper::{Float, RoundingMode, F32};

fn decode_rm(rm: usize) -> Option<RoundingMode> {
    match rm {
        0b000 => Some(RoundingMode::TiesToEven), // RNE, Round to Nearest, ties to Even
        0b001 => Some(RoundingMode::TowardZero), // RTZ, Round towards Zero
        0b010 => Some(RoundingMode::TowardNegative), // RDN, Round Down (towards −∞)
        0b011 => Some(RoundingMode::TowardPositive), // RUP, Round Up (towards +∞)
        0b100 => Some(RoundingMode::TiesToAway), // RMM, Round to Nearest, ties to Max Magnitude
        _ => None,
    }
}

fn select_rm(rm: usize, csr: &mut ControlAndStatusRegister) -> Option<RoundingMode> {
    match rm {
        0b111 => {
            // DYN, In instruction’s rm field, selects dynamic rounding mode; In Rounding Mode register, Invalid.
            let frm = ((csr.csrrs(FCSR, 0) >> 5) & MASK_3BIT) as usize;
            decode_rm(frm)
        }
        _ => decode_rm(rm),
    }
}

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
        f: &mut FloatingPointRegister,
        csr: &mut ControlAndStatusRegister,
        _: &mut SystemBus,
    ) {
        match instruction {
            Instruction::TypeR {
                opcode,
                rd,
                funct3,
                rs1,
                rs2,
                funct7,
            } => {
                // TODO: handle invalid
                let rm = select_rm(funct3, csr).unwrap();
                let rs3 = (funct7 >> 2) & MASK_5BIT as usize;
                match opcode {
                    Rv32fOpcodeR::FmaddS => f.write(
                        rd,
                        F32::from_bits(f.read(rs1))
                            .fused_mul_add(
                                F32::from_bits(f.read(rs2)),
                                F32::from_bits(f.read(rs3)),
                                rm,
                            )
                            .to_bits(),
                    ),
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
                }
            }
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
