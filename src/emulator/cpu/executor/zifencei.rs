use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister, executor::Executor, f::FloatingPointRegister,
            pc::ProgramCounter, x::IntegerRegister,
        },
    },
    isa::instruction::{
        zifencei::{
            ZifenceiOpcodeB, ZifenceiOpcodeI, ZifenceiOpcodeJ, ZifenceiOpcodeR, ZifenceiOpcodeS,
            ZifenceiOpcodeU,
        },
        Instruction,
    },
};

pub struct ZifenceiExecutor;

impl Executor for ZifenceiExecutor {
    type OpcodeR = ZifenceiOpcodeR;
    type OpcodeI = ZifenceiOpcodeI;
    type OpcodeS = ZifenceiOpcodeS;
    type OpcodeB = ZifenceiOpcodeB;
    type OpcodeU = ZifenceiOpcodeU;
    type OpcodeJ = ZifenceiOpcodeJ;

    fn execute(
        instruction: Instruction<
            ZifenceiOpcodeR,
            ZifenceiOpcodeI,
            ZifenceiOpcodeS,
            ZifenceiOpcodeB,
            ZifenceiOpcodeU,
            ZifenceiOpcodeJ,
        >,
        _: &mut ProgramCounter,
        _: &mut IntegerRegister,
        _: &mut FloatingPointRegister,
        _: &mut ControlAndStatusRegister,
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
                rs1: _,
                rd: _,
                imm: _,
            } => match opcode {
                ZifenceiOpcodeI::FenceI => {} // not yet supported
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
