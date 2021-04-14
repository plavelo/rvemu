use crate::{
    emulator::cpu::decoder::{Decoder, MASK_3BIT, MASK_7BIT},
    isa::instruction::{
        rv32i::{
            Rv32iOpcodeB, Rv32iOpcodeI, Rv32iOpcodeJ, Rv32iOpcodeR, Rv32iOpcodeS, Rv32iOpcodeU,
        },
        Instruction,
    },
};

pub struct Rv32iDecoder;

impl Decoder for Rv32iDecoder {
    type OpcodeR = Rv32iOpcodeR;
    type OpcodeI = Rv32iOpcodeI;
    type OpcodeS = Rv32iOpcodeS;
    type OpcodeB = Rv32iOpcodeB;
    type OpcodeU = Rv32iOpcodeU;
    type OpcodeJ = Rv32iOpcodeJ;

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
        let funct7 = (instruction >> 25) & MASK_7BIT;
        match opcode {
            0b0110111 => Self::decode_u(Some(Rv32iOpcodeU::Lui), instruction),
            0b0010111 => Self::decode_u(Some(Rv32iOpcodeU::Auipc), instruction),
            0b1101111 => Self::decode_j(Some(Rv32iOpcodeJ::Jal), instruction),
            0b1100111 => Self::decode_i(Some(Rv32iOpcodeI::Jalr), instruction),
            0b1100011 => Self::decode_b(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeB::Beq),
                    0b001 => Some(Rv32iOpcodeB::Bne),
                    0b100 => Some(Rv32iOpcodeB::Blt),
                    0b101 => Some(Rv32iOpcodeB::Bge),
                    0b110 => Some(Rv32iOpcodeB::Bltu),
                    0b111 => Some(Rv32iOpcodeB::Bgeu),
                    _ => None,
                },
                instruction,
            ),
            0b0000011 => Self::decode_i(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeI::Lb),
                    0b001 => Some(Rv32iOpcodeI::Lh),
                    0b010 => Some(Rv32iOpcodeI::Lw),
                    0b100 => Some(Rv32iOpcodeI::Lbu),
                    0b101 => Some(Rv32iOpcodeI::Lhu),
                    _ => None,
                },
                instruction,
            ),
            0b0100011 => Self::decode_s(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeS::Sb),
                    0b001 => Some(Rv32iOpcodeS::Sh),
                    0b010 => Some(Rv32iOpcodeS::Sw),
                    _ => None,
                },
                instruction,
            ),
            0b0010011 => Self::decode_i(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeI::Addi),
                    0b010 => Some(Rv32iOpcodeI::Slti),
                    0b011 => Some(Rv32iOpcodeI::Sltiu),
                    0b100 => Some(Rv32iOpcodeI::Xori),
                    0b110 => Some(Rv32iOpcodeI::Ori),
                    0b111 => Some(Rv32iOpcodeI::Andi),
                    0b001 => Some(Rv32iOpcodeI::Slli),
                    0b101 => match funct7 {
                        0b0000000 => Some(Rv32iOpcodeI::Srli),
                        0b0100000 => Some(Rv32iOpcodeI::Srai),
                        _ => None,
                    },
                    _ => None,
                },
                instruction,
            ),
            0b0110011 => Self::decode_r(
                match funct3 {
                    0b000 => match funct7 {
                        0b0000000 => Some(Rv32iOpcodeR::Add),
                        0b0100000 => Some(Rv32iOpcodeR::Sub),
                        _ => None,
                    },
                    0b001 => Some(Rv32iOpcodeR::Sll),
                    0b010 => Some(Rv32iOpcodeR::Slt),
                    0b011 => Some(Rv32iOpcodeR::Sltu),
                    0b100 => Some(Rv32iOpcodeR::Xor),
                    0b101 => match funct7 {
                        0b0000000 => Some(Rv32iOpcodeR::Srl),
                        0b0100000 => Some(Rv32iOpcodeR::Sra),
                        _ => None,
                    },
                    0b110 => Some(Rv32iOpcodeR::Or),
                    0b111 => Some(Rv32iOpcodeR::And),
                    _ => None,
                },
                instruction,
            ),
            0b0001111 => Self::decode_i(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeI::Fence),
                    _ => None,
                },
                instruction,
            ),
            0b1110011 => Self::decode_i(
                match funct3 {
                    0b000 => match instruction >> 20 {
                        0b0 => Some(Rv32iOpcodeI::Ecall),
                        0b1 => Some(Rv32iOpcodeI::Ebreak),
                        _ => None,
                    },
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_type_r_ok() {
        let inst = 0b0000000_00101_01010_000_10101_0110011;
        assert_eq!(
            Rv32iDecoder::decode(inst).unwrap(),
            Instruction::TypeR {
                opcode: Rv32iOpcodeR::Add,
                rd: 0b10101,
                funct3: 0,
                rs1: 0b01010,
                rs2: 0b00101,
                funct7: 0b0,
            }
        );
    }

    #[test]
    fn decode_type_i_ok() {
        let inst = 0b100000000101_01010_000_00101_1100111;
        assert_eq!(
            Rv32iDecoder::decode(inst).unwrap(),
            Instruction::TypeI {
                opcode: Rv32iOpcodeI::Jalr,
                rd: 0b00101,
                funct3: 0,
                rs1: 0b01010,
                imm: 0b11111111111111111111111111111111_11111111111111111111100000000101,
            }
        );
    }

    #[test]
    fn decode_type_s_ok() {
        let inst = 0b1010101_00101_01010_000_10101_0100011;
        assert_eq!(
            Rv32iDecoder::decode(inst).unwrap(),
            Instruction::TypeS {
                opcode: Rv32iOpcodeS::Sb,
                funct3: 0,
                rs1: 0b01010,
                rs2: 0b00101,
                imm: 0b11111111111111111111111111111111_11111111111111111111_1010101_10101,
            }
        );
    }

    #[test]
    fn decode_type_b_ok() {
        let inst = 0b1010101_00101_01010_000_10101_1100011;
        assert_eq!(
            Rv32iDecoder::decode(inst).unwrap(),
            Instruction::TypeB {
                opcode: Rv32iOpcodeB::Beq,
                funct3: 0,
                rs1: 0b01010,
                rs2: 0b00101,
                imm: 0b11111111111111111111111111111111_11111111111111111111_1_010101_1010_0,
            }
        );
    }

    #[test]
    fn decode_type_u_ok() {
        let inst = 0b00000000010101010101_00101_0010111;
        assert_eq!(
            Rv32iDecoder::decode(inst).unwrap(),
            Instruction::TypeU {
                opcode: Rv32iOpcodeU::Auipc,
                rd: 0b00101,
                imm: 0b00000000000000000000000000000000_010101010101_00000_0000000,
            }
        );
    }

    #[test]
    fn decode_type_j_ok() {
        let inst = 0b1_0000000010_1_01010101_00101_1101111;
        assert_eq!(
            Rv32iDecoder::decode(inst).unwrap(),
            Instruction::TypeJ {
                opcode: Rv32iOpcodeJ::Jal,
                rd: 0b00101,
                imm: 0b11111111111111111111111111111111_11111111111_1_01010101_1_0000000010_0,
            }
        );
    }
}
