pub mod privileged;
pub mod rv32f;
pub mod rv32i;
pub mod rv32m;
pub mod rv64i;
pub mod rv64m;
pub mod zicsr;
pub mod zifencei;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Instruction<OpcodeR, OpcodeI, OpcodeS, OpcodeB, OpcodeU, OpcodeJ> {
    TypeR {
        opcode: OpcodeR,
        rs1: usize,
        rs2: usize,
        rd: usize,
    },
    TypeI {
        opcode: OpcodeI,
        rs1: usize,
        rd: usize,
        imm: u64,
    },
    TypeS {
        opcode: OpcodeS,
        rs1: usize,
        rs2: usize,
        imm: u64,
    },
    TypeB {
        opcode: OpcodeB,
        rs1: usize,
        rs2: usize,
        imm: u64,
    },
    TypeU {
        opcode: OpcodeU,
        rd: usize,
        imm: u64,
    },
    TypeJ {
        opcode: OpcodeJ,
        rd: usize,
        imm: u64,
    },
}

impl<
        OpcodeR: fmt::Display,
        OpcodeI: fmt::Display,
        OpcodeS: fmt::Display,
        OpcodeB: fmt::Display,
        OpcodeU: fmt::Display,
        OpcodeJ: fmt::Display,
    > fmt::Display for Instruction<OpcodeR, OpcodeI, OpcodeS, OpcodeB, OpcodeU, OpcodeJ>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::TypeR {
                opcode,
                rs1,
                rs2,
                rd,
            } => write!(
                f,
                "Instruction::TypeR {{ opcode: {}, rs1: {}, rs2: {}, rd: {} }}",
                opcode, rs1, rs2, rd,
            ),
            Instruction::TypeI {
                opcode,
                rs1,
                rd,
                imm,
            } => write!(
                f,
                "Instruction::TypeI {{ opcode: {}, rs1: {}, rd: {}, imm: {} }}",
                opcode, rs1, rd, imm,
            ),
            Instruction::TypeS {
                opcode,
                rs1,
                rs2,
                imm,
            } => write!(
                f,
                "Instruction::TypeS {{ opcode: {}, rs1: {}, rs2: {}, imm: {} }}",
                opcode, rs1, rs2, imm,
            ),
            Instruction::TypeB {
                opcode,
                rs1,
                rs2,
                imm,
            } => write!(
                f,
                "Instruction::TypeB {{ opcode: {}, rs1: {}, rs2: {}, imm: {} }}",
                opcode, rs1, rs2, imm,
            ),
            Instruction::TypeU { opcode, rd, imm } => write!(
                f,
                "Instruction::TypeU {{ opcode: {}, rd: {}, imm: {} }}",
                opcode, rd, imm,
            ),
            Instruction::TypeJ { opcode, rd, imm } => write!(
                f,
                "Instruction::TypeJ {{ opcode: {}, rd: {}, imm: {} }}",
                opcode, rd, imm,
            ),
        }
    }
}
