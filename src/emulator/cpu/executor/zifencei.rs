use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister, executor::Executor, f::FloatingPointRegister,
            pc::ProgramCounter, x::IntegerRegister,
        },
    },
    isa::{
        instruction::{
            zifencei::{
                ZifenceiOpcodeB, ZifenceiOpcodeI, ZifenceiOpcodeJ, ZifenceiOpcodeR,
                ZifenceiOpcodeS, ZifenceiOpcodeU,
            },
            Instruction,
        },
        privileged::{cause::Cause, mode::PrivilegeMode},
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
        _: &PrivilegeMode,
        _: &mut ProgramCounter,
        _: &mut IntegerRegister,
        _: &mut FloatingPointRegister,
        _: &mut ControlAndStatusRegister,
        _: &mut SystemBus,
    ) -> Result<(), Cause> {
        if let Instruction::TypeI {
            opcode,
            rd: _,
            funct3: _,
            rs1: _,
            imm: _,
        } = instruction
        {
            match opcode {
                ZifenceiOpcodeI::FenceI => Ok(()), // not yet supported
            }
        } else {
            Ok(())
        }
    }
}
