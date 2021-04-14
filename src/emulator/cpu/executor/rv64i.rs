use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister,
            executor::{Executor, MASK_5BIT},
            f::FloatingPointRegister,
            pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::{
        instruction::{
            rv64i::{
                Rv64iOpcodeB, Rv64iOpcodeI, Rv64iOpcodeJ, Rv64iOpcodeR, Rv64iOpcodeS, Rv64iOpcodeU,
            },
            Instruction,
        },
        privileged::{cause::Cause, mode::PrivilegeMode},
    },
};

pub struct Rv64iExecutor;

impl Executor for Rv64iExecutor {
    type OpcodeR = Rv64iOpcodeR;
    type OpcodeI = Rv64iOpcodeI;
    type OpcodeS = Rv64iOpcodeS;
    type OpcodeB = Rv64iOpcodeB;
    type OpcodeU = Rv64iOpcodeU;
    type OpcodeJ = Rv64iOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv64iOpcodeR,
            Rv64iOpcodeI,
            Rv64iOpcodeS,
            Rv64iOpcodeB,
            Rv64iOpcodeU,
            Rv64iOpcodeJ,
        >,
        _: &PrivilegeMode,
        _: &mut ProgramCounter,
        x: &mut IntegerRegister,
        _: &mut FloatingPointRegister,
        _: &mut ControlAndStatusRegister,
        bus: &mut SystemBus,
    ) -> Result<(), Cause> {
        match instruction {
            Instruction::TypeR {
                opcode,
                rd,
                funct3: _,
                rs1,
                rs2,
                funct7: _,
            } => match opcode {
                Rv64iOpcodeR::Sllw => x.writei(
                    rd,
                    ((x.readu(rs1) as u32) << (x.readu(rs2) & MASK_5BIT)) as i32 as i64,
                ),
                Rv64iOpcodeR::Srlw => x.writei(
                    rd,
                    ((x.readu(rs1) as u32) >> (x.readu(rs2) & MASK_5BIT)) as i32 as i64,
                ),
                Rv64iOpcodeR::Sraw => x.writei(
                    rd,
                    ((x.readi(rs1) as i32) >> (x.readu(rs2) & MASK_5BIT)) as i64,
                ),
                Rv64iOpcodeR::Addw => {
                    x.writei(rd, x.readu(rs1).wrapping_add(x.readu(rs2)) as i32 as i64)
                }
                Rv64iOpcodeR::Subw => {
                    x.writei(rd, x.readu(rs1).wrapping_sub(x.readu(rs2)) as i32 as i64)
                }
            },
            Instruction::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv64iOpcodeI::Slliw => x.writei(
                    rd,
                    ((x.readu(rs1) as u32) << (imm & MASK_5BIT)) as i32 as i64,
                ),
                Rv64iOpcodeI::Srliw => x.writei(
                    rd,
                    ((x.readu(rs1) as u32) >> (imm & MASK_5BIT)) as i32 as i64,
                ),
                Rv64iOpcodeI::Sraiw => {
                    x.writei(rd, ((x.readi(rs1) as i32) >> (imm & MASK_5BIT)) as i64)
                }
                Rv64iOpcodeI::Addiw => x.writei(rd, x.readu(rs1).wrapping_add(imm) as i32 as i64),
                Rv64iOpcodeI::Lwu => x.writeu(
                    rd,
                    bus.load32(x.readi(rs1).wrapping_add(imm as i64) as u64) as u64,
                ),
                Rv64iOpcodeI::Ld => {
                    x.writeu(rd, bus.load64(x.readi(rs1).wrapping_add(imm as i64) as u64))
                }
            },
            Instruction::TypeS {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv64iOpcodeS::Sd => {
                    bus.store64(x.readi(rs1).wrapping_add(imm as i64) as u64, x.readu(rs2))
                }
            },
            _ => (),
        }
        Ok(())
    }
}
