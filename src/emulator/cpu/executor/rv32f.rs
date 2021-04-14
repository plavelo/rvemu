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
        privileged::{cause::Cause, mode::PrivilegeMode},
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
        _: &PrivilegeMode,
        _: &mut ProgramCounter,
        x: &mut IntegerRegister,
        f: &mut FloatingPointRegister,
        csr: &mut ControlAndStatusRegister,
        bus: &mut SystemBus,
    ) -> Result<(), Cause> {
        match instruction {
            Instruction::TypeR {
                opcode,
                rd,
                funct3,
                rs1,
                rs2,
                funct7,
            } => {
                // TODO: NaN handling
                // TODO: invalid rm handling
                let rm = select_rm(funct3, csr).unwrap();
                let rs3 = (funct7 >> 2) & MASK_5BIT as usize;
                match opcode {
                    Rv32fOpcodeR::FmaddS => {
                        f.write(
                            rd,
                            F32::from_bits(f.read(rs1))
                                .fused_mul_add(
                                    F32::from_bits(f.read(rs2)),
                                    F32::from_bits(f.read(rs3)),
                                    rm,
                                )
                                .to_bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FmsubS => {
                        f.write(
                            rd,
                            F32::from_bits(f.read(rs1))
                                .fused_mul_add(
                                    F32::from_bits(f.read(rs2)),
                                    F32::neg(&F32::from_bits(f.read(rs3))),
                                    rm,
                                )
                                .to_bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FnmsubS => {
                        f.write(
                            rd,
                            F32::from_bits(f.read(rs1))
                                .fused_mul_add(
                                    F32::neg(&F32::from_bits(f.read(rs2))),
                                    F32::neg(&F32::from_bits(f.read(rs3))),
                                    rm,
                                )
                                .to_bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FnmaddS => {
                        f.write(
                            rd,
                            F32::from_bits(f.read(rs1))
                                .fused_mul_add(
                                    F32::neg(&F32::from_bits(f.read(rs2))),
                                    F32::from_bits(f.read(rs3)),
                                    rm,
                                )
                                .to_bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FaddS => {
                        f.write(
                            rd,
                            F32::from_bits(f.read(rs1))
                                .add(F32::from_bits(f.read(rs2)), rm)
                                .to_bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FsubS => {
                        f.write(
                            rd,
                            F32::from_bits(f.read(rs1))
                                .sub(F32::from_bits(f.read(rs2)), rm)
                                .to_bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FmulS => {
                        f.write(
                            rd,
                            F32::from_bits(f.read(rs1))
                                .mul(F32::from_bits(f.read(rs2)), rm)
                                .to_bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FdivS => {
                        f.write(
                            rd,
                            F32::from_bits(f.read(rs1))
                                .div(F32::from_bits(f.read(rs2)), rm)
                                .to_bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FsqrtS => {
                        f.write(rd, F32::from_bits(f.read(rs1)).sqrt(rm).to_bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FsgnjS => {
                        let sign = F32::from_bits(f.read(rs2)).sign();
                        let mut ret = F32::from_bits(f.read(rs1));
                        ret.set_sign(sign);
                        f.write(rd, ret.to_bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FsgnjnS => {
                        let sign = F32::from_bits(f.read(rs2)).sign();
                        let mut ret = F32::from_bits(f.read(rs1));
                        ret.set_sign(!sign);
                        f.write(rd, ret.to_bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FsgnjxS => {
                        let mut ret = F32::from_bits(f.read(rs1));
                        let sign = F32::from_bits(f.read(rs2)).sign() ^ ret.sign();
                        ret.set_sign(sign);
                        f.write(rd, ret.to_bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FminS => {
                        let val1 = F32::from_bits(f.read(rs1));
                        let val2 = F32::from_bits(f.read(rs2));
                        let min = if val1.lt(val2) { val1 } else { val2 };
                        f.write(rd, min.to_bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FmaxS => {
                        let val1 = F32::from_bits(f.read(rs1));
                        let val2 = F32::from_bits(f.read(rs2));
                        let max = if val1.lt(val2) { val2 } else { val1 };
                        f.write(rd, max.to_bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FcvtWs => {
                        x.writei(rd, F32::from_bits(f.read(rs1)).to_i32(rm, false) as i64);
                        Ok(())
                    }
                    Rv32fOpcodeR::FcvtWuS => {
                        x.writei(
                            rd,
                            F32::from_bits(f.read(rs1)).to_u32(rm, false) as i32 as i64,
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FmvXw => {
                        x.writei(rd, f.read(rs1) as i32 as i64);
                        Ok(())
                    }
                    Rv32fOpcodeR::FeqS => {
                        let val1 = F32::from_bits(f.read(rs1));
                        let val2 = F32::from_bits(f.read(rs2));
                        let ret = if val1.eq(val2) { 1 } else { 0 };
                        f.write(rd, ret);
                        Ok(())
                    }
                    Rv32fOpcodeR::FltS => {
                        let val1 = F32::from_bits(f.read(rs1));
                        let val2 = F32::from_bits(f.read(rs2));
                        let ret = if val1.lt(val2) { 1 } else { 0 };
                        f.write(rd, ret);
                        Ok(())
                    }
                    Rv32fOpcodeR::FleS => {
                        let val1 = F32::from_bits(f.read(rs1));
                        let val2 = F32::from_bits(f.read(rs2));
                        let ret = if val1.le(val2) { 1 } else { 0 };
                        f.write(rd, ret);
                        Ok(())
                    }
                    Rv32fOpcodeR::FclassS => {
                        let val = F32::from_bits(f.read(rs1));
                        let class = if val.is_negative_infinity() {
                            0
                        } else if val.is_negative_normal() {
                            1
                        } else if val.is_negative_subnormal() {
                            2
                        } else if val.is_negative_zero() {
                            3
                        } else if val.is_positive_zero() {
                            4
                        } else if val.is_positive_subnormal() {
                            5
                        } else if val.is_positive_normal() {
                            6
                        } else if val.is_positive_infinity() {
                            7
                        } else {
                            // TODO: supports signaling NaN / quiet NaN
                            9
                        };
                        x.writeu(rd, class);
                        Ok(())
                    }
                    Rv32fOpcodeR::FcvtSw => {
                        f.write(rd, F32::from_i32(x.readi(rs1) as i32, rm).to_bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FcvtSWu => {
                        f.write(rd, F32::from_u32(x.readi(rs1) as u32, rm).to_bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FmvWx => {
                        f.write(rd, x.readu(rs1) as u32);
                        Ok(())
                    }
                }
            }
            Instruction::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv32fOpcodeI::Flw => {
                    f.write(
                        rd,
                        bus.load32(x.readi(rs1).wrapping_add(imm as i64) as u64) as u32,
                    );
                    Ok(())
                }
            },
            Instruction::TypeS {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32fOpcodeS::Fsw => {
                    bus.store32(x.readi(rs1).wrapping_add(imm as i64) as u64, f.read(rs2));
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }
}
