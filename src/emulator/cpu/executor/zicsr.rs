use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister,
            executor::{Executor, MASK_12BIT},
            f::FloatingPointRegister,
            pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::instruction::{
        zicsr::{
            ZicsrOpcodeB, ZicsrOpcodeI, ZicsrOpcodeJ, ZicsrOpcodeR, ZicsrOpcodeS, ZicsrOpcodeU,
        },
        Instruction,
    },
};

pub struct ZicsrExecutor;

impl Executor for ZicsrExecutor {
    type OpcodeR = ZicsrOpcodeR;
    type OpcodeI = ZicsrOpcodeI;
    type OpcodeS = ZicsrOpcodeS;
    type OpcodeB = ZicsrOpcodeB;
    type OpcodeU = ZicsrOpcodeU;
    type OpcodeJ = ZicsrOpcodeJ;

    fn execute(
        instruction: Instruction<
            ZicsrOpcodeR,
            ZicsrOpcodeI,
            ZicsrOpcodeS,
            ZicsrOpcodeB,
            ZicsrOpcodeU,
            ZicsrOpcodeJ,
        >,
        _: &mut ProgramCounter,
        x: &mut IntegerRegister,
        _: &mut FloatingPointRegister,
        csr: &mut ControlAndStatusRegister,
        _: &mut SystemBus,
    ) {
        match instruction {
            Instruction::TypeR {
                opcode: _,
                rs1: _,
                rs2: _,
                rd: _,
            } => {}
            Instruction::TypeI {
                opcode,
                rs1,
                rd,
                imm,
            } => match opcode {
                ZicsrOpcodeI::Csrrw => x.writeu(rd, csr.csrrw(imm & MASK_12BIT, x.readu(rs1))),
                ZicsrOpcodeI::Csrrs => x.writeu(rd, csr.csrrs(imm & MASK_12BIT, x.readu(rs1))),
                ZicsrOpcodeI::Csrrc => x.writeu(rd, csr.csrrc(imm & MASK_12BIT, x.readu(rs1))),
                ZicsrOpcodeI::Csrrwi => x.writeu(rd, csr.csrrw(imm & MASK_12BIT, rs1 as u64)),
                ZicsrOpcodeI::Csrrsi => x.writeu(rd, csr.csrrs(imm & MASK_12BIT, rs1 as u64)),
                ZicsrOpcodeI::Csrrci => x.writeu(rd, csr.csrrc(imm & MASK_12BIT, rs1 as u64)),
            },
            Instruction::TypeS {
                opcode: _,
                rs1: _,
                rs2: _,
                imm: _,
            } => {}
            Instruction::TypeB {
                opcode: _,
                rs1: _,
                rs2: _,
                imm: _,
            } => {}
            Instruction::TypeU {
                opcode: _,
                rd: _,
                imm: _,
            } => {}
            Instruction::TypeJ {
                opcode: _,
                rd: _,
                imm: _,
            } => {}
        }
    }
}
