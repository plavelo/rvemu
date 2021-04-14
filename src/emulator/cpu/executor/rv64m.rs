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
            rv64m::{
                Rv64mOpcodeB, Rv64mOpcodeI, Rv64mOpcodeJ, Rv64mOpcodeR, Rv64mOpcodeS, Rv64mOpcodeU,
            },
            Instruction,
        },
        privileged::{cause::Cause, mode::PrivilegeMode},
    },
};

pub struct Rv64mExecutor;

impl Executor for Rv64mExecutor {
    type OpcodeR = Rv64mOpcodeR;
    type OpcodeI = Rv64mOpcodeI;
    type OpcodeS = Rv64mOpcodeS;
    type OpcodeB = Rv64mOpcodeB;
    type OpcodeU = Rv64mOpcodeU;
    type OpcodeJ = Rv64mOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv64mOpcodeR,
            Rv64mOpcodeI,
            Rv64mOpcodeS,
            Rv64mOpcodeB,
            Rv64mOpcodeU,
            Rv64mOpcodeJ,
        >,
        _: &PrivilegeMode,
        _: &mut ProgramCounter,
        x: &mut IntegerRegister,
        _: &mut FloatingPointRegister,
        _: &mut ControlAndStatusRegister,
        _: &mut SystemBus,
    ) -> Result<(), Cause> {
        if let Instruction::TypeR {
            opcode,
            rd,
            funct3: _,
            rs1,
            rs2,
            funct7: _,
        } = instruction
        {
            match opcode {
                Rv64mOpcodeR::Mulw => x.writei(
                    rd,
                    x.readu(rs1).wrapping_mul(x.readu(rs2)) as u32 as i32 as i64,
                ),
                Rv64mOpcodeR::Divw => {
                    let dividend = x.readi(rs1) as i32;
                    let divisor = x.readi(rs2) as i32;
                    x.writei(
                        rd,
                        if divisor == 0 {
                            i64::MAX
                        } else {
                            dividend.wrapping_div(divisor) as i64
                        },
                    )
                }
                Rv64mOpcodeR::Divuw => {
                    let dividend = x.readu(rs1) as u32;
                    let divisor = x.readu(rs2) as u32;
                    x.writei(
                        rd,
                        if divisor == 0 {
                            i64::MAX
                        } else {
                            dividend.wrapping_div(divisor) as i32 as i64
                        },
                    )
                }
                Rv64mOpcodeR::Remw => {
                    let dividend = x.readi(rs1);
                    let divisor = x.readi(rs2);
                    x.writei(
                        rd,
                        if divisor == 0 {
                            dividend
                        } else {
                            (dividend as i32).wrapping_rem(divisor as i32) as i64
                        },
                    )
                }
                Rv64mOpcodeR::Remuw => {
                    let dividend = x.readu(rs1);
                    let divisor = x.readu(rs2);
                    x.writei(
                        rd,
                        if divisor == 0 {
                            dividend as i64
                        } else {
                            (dividend as u32).wrapping_rem(divisor as u32) as i32 as i64
                        },
                    )
                }
            }
        }
        Ok(())
    }
}
