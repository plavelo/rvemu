pub mod privileged;
pub mod rv32f;
pub mod rv32i;
pub mod rv32m;
pub mod rv64i;
pub mod rv64m;
pub mod zicsr;
pub mod zifencei;

use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister, f::FloatingPointRegister, pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::{
        instruction::Instruction,
        privileged::{cause::Cause, mode::PrivilegeMode},
    },
};

const MASK_3BIT: u64 = 0b111;
const MASK_5BIT: u64 = 0b11111;
const MASK_12BIT: u64 = 0b111111111111;

pub trait Executor {
    type OpcodeR;
    type OpcodeI;
    type OpcodeS;
    type OpcodeB;
    type OpcodeU;
    type OpcodeJ;

    #[allow(clippy::type_complexity)]
    fn execute(
        instruction: Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
        prv: &PrivilegeMode,
        pc: &mut ProgramCounter,
        x: &mut IntegerRegister,
        f: &mut FloatingPointRegister,
        csr: &mut ControlAndStatusRegister,
        bus: &mut SystemBus,
    ) -> Result<(), Cause>;
}
