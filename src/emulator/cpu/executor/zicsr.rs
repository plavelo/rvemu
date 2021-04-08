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
    isa::{
        instruction::{
            zicsr::{
                ZicsrOpcodeB, ZicsrOpcodeI, ZicsrOpcodeJ, ZicsrOpcodeR, ZicsrOpcodeS, ZicsrOpcodeU,
            },
            Instruction,
        },
        privileged::{cause::Cause, mode::PrivilegeMode},
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
        _: &PrivilegeMode,
        _: &mut ProgramCounter,
        x: &mut IntegerRegister,
        _: &mut FloatingPointRegister,
        csr: &mut ControlAndStatusRegister,
        _: &mut SystemBus,
    ) -> Result<(), Cause> {
        if let Instruction::TypeI {
            opcode,
            rd,
            funct3: _,
            rs1,
            imm,
        } = instruction
        {
            match opcode {
                ZicsrOpcodeI::Csrrw => Ok(x.writeu(rd, csr.csrrw(imm & MASK_12BIT, x.readu(rs1)))),
                ZicsrOpcodeI::Csrrs => Ok(x.writeu(rd, csr.csrrs(imm & MASK_12BIT, x.readu(rs1)))),
                ZicsrOpcodeI::Csrrc => Ok(x.writeu(rd, csr.csrrc(imm & MASK_12BIT, x.readu(rs1)))),
                ZicsrOpcodeI::Csrrwi => Ok(x.writeu(rd, csr.csrrw(imm & MASK_12BIT, rs1 as u64))),
                ZicsrOpcodeI::Csrrsi => Ok(x.writeu(rd, csr.csrrs(imm & MASK_12BIT, rs1 as u64))),
                ZicsrOpcodeI::Csrrci => Ok(x.writeu(rd, csr.csrrc(imm & MASK_12BIT, rs1 as u64))),
            }
        } else {
            Ok(())
        }
    }
}
