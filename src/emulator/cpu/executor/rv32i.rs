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
            rv32i::{
                Rv32iOpcodeB, Rv32iOpcodeI, Rv32iOpcodeJ, Rv32iOpcodeR, Rv32iOpcodeS, Rv32iOpcodeU,
            },
            Instruction,
        },
        privileged::cause::Cause,
    },
};

pub struct Rv32iExecutor;

impl Executor for Rv32iExecutor {
    type OpcodeR = Rv32iOpcodeR;
    type OpcodeI = Rv32iOpcodeI;
    type OpcodeS = Rv32iOpcodeS;
    type OpcodeB = Rv32iOpcodeB;
    type OpcodeU = Rv32iOpcodeU;
    type OpcodeJ = Rv32iOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv32iOpcodeR,
            Rv32iOpcodeI,
            Rv32iOpcodeS,
            Rv32iOpcodeB,
            Rv32iOpcodeU,
            Rv32iOpcodeJ,
        >,
        pc: &mut ProgramCounter,
        x: &mut IntegerRegister,
        _: &mut FloatingPointRegister,
        csr: &mut ControlAndStatusRegister,
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
                Rv32iOpcodeR::Sll => Ok(x.writeu(rd, x.readu(rs1) << (x.readu(rs2) & MASK_5BIT))),
                Rv32iOpcodeR::Srl => Ok(x.writeu(rd, x.readu(rs1) >> (x.readu(rs2) & MASK_5BIT))),
                Rv32iOpcodeR::Sra => Ok(x.writei(rd, x.readi(rs1) >> (x.readu(rs2) & MASK_5BIT))),
                Rv32iOpcodeR::Add => Ok(x.writeu(rd, x.readu(rs1).wrapping_add(x.readu(rs2)))),
                Rv32iOpcodeR::Sub => Ok(x.writeu(rd, x.readu(rs1).wrapping_sub(x.readu(rs2)))),
                Rv32iOpcodeR::Xor => Ok(x.writeu(rd, x.readu(rs1) ^ x.readu(rs2))),
                Rv32iOpcodeR::Or => Ok(x.writeu(rd, x.readu(rs1) | x.readu(rs2))),
                Rv32iOpcodeR::And => Ok(x.writeu(rd, x.readu(rs1) & x.readu(rs2))),
                Rv32iOpcodeR::Slt => {
                    Ok(x.writeu(rd, if x.readi(rs1) < x.readi(rs2) { 1 } else { 0 }))
                }
                Rv32iOpcodeR::Sltu => {
                    Ok(x.writeu(rd, if x.readu(rs1) < x.readu(rs2) { 1 } else { 0 }))
                }
            },
            Instruction::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv32iOpcodeI::Slli => Ok(x.writeu(rd, x.readu(rs1) << (imm & MASK_5BIT))),
                Rv32iOpcodeI::Srli => Ok(x.writeu(rd, x.readu(rs1) >> (imm & MASK_5BIT))),
                Rv32iOpcodeI::Srai => Ok(x.writei(rd, x.readi(rs1) >> (imm & MASK_5BIT))),
                Rv32iOpcodeI::Addi => Ok(x.writeu(rd, x.readu(rs1).wrapping_add(imm))),
                Rv32iOpcodeI::Xori => Ok(x.writeu(rd, x.readu(rs1) ^ imm)),
                Rv32iOpcodeI::Ori => Ok(x.writeu(rd, x.readu(rs1) | imm)),
                Rv32iOpcodeI::Andi => Ok(x.writeu(rd, x.readu(rs1) & imm)),
                Rv32iOpcodeI::Slti => {
                    Ok(x.writeu(rd, if x.readi(rs1) < imm as i64 { 1 } else { 0 }))
                }
                Rv32iOpcodeI::Sltiu => Ok(x.writeu(rd, if x.readu(rs1) < imm { 1 } else { 0 })),
                Rv32iOpcodeI::Jalr => {
                    let last = pc.read();
                    pc.jump((x.readi(rs1).wrapping_add(imm as i64) & !1) as u64);
                    Ok(x.writeu(rd, last.wrapping_add(4)))
                }
                Rv32iOpcodeI::Fence => Ok(()), // not yet supported
                Rv32iOpcodeI::Ecall => {
                    // これらのハンドリングはどこか一箇所でやったほうが良さそうな気がする
                    //  0. ExceptionCode::EnvironmentCallFromUmode = 8,EnvironmentCallFromSmode = 9,EnvironmentCallFromMmode = 11,をチェックして現在の特権レベルからどの特権レベルに遷移するか確認
                    //  1. U-modeにてtrapが発生21した。trapの節で述べたようにsstatus.SIEのbit値に関わらず発生する。
                    //  2. scauseにtrapの発生原因が入る。例えばecallの場合、bit31(Interrupt)=0, Exception code(bit3-0)=4b1000=8となる
                    //  3. sepcに1で発生した箇所のprogram counterが入る22
                    //  4. stvalにexception-specific valueが入る(例えば、page-fault exceptionだったら、page faultが発生したvirtual addressが格納される)
                    //  5. sstatus.SPP(S-mode Privious Privilege) <~ U-mode(00)に設定
                    //  6. sstatus.SPIE <~ sstatus.SIE(Software Interrupt Enable) [SIEをsave]
                    //  7. sstatus.SIE <~ 0 [always]
                    //  8. S-modeに遷移
                    //  9. pc <~ stvecの関数アドレス(uservec関数)にセットされる
                    // 10. [trap handlerの処理; software処理開始]
                    // 11. integer and floating-point registers達をsscratch CSRに退避し、S-modeで使うべきregisterをrestoreする(uservec関数)
                    // 12. 次のtrapに備えて、proc構造体sscratchから9のinteger and floating-point registers達をrestoreする(userret関数)
                    // 13. sret実行
                    // 14. sstatus.SIE <- sstatus.SPIE(=1)
                    // 15. U-modeに遷移する
                    // 16. sstatus.SPIE <~ 1 [always]
                    // 17. sstatus.SPP <~ 00(U-mode) [always]
                    // 18. pc(program counter) <~ sepc CSR
                    Ok(())
                } // not yet supported
                Rv32iOpcodeI::Ebreak => Ok(()), // not yet supported
                Rv32iOpcodeI::Lb => Ok(x.writei(
                    rd,
                    bus.load8(x.readi(rs1).wrapping_add(imm as i64) as u64) as i64,
                )),
                Rv32iOpcodeI::Lh => Ok(x.writei(
                    rd,
                    bus.load16(x.readi(rs1).wrapping_add(imm as i64) as u64) as i64,
                )),
                Rv32iOpcodeI::Lbu => Ok(x.writeu(
                    rd,
                    bus.load8(x.readi(rs1).wrapping_add(imm as i64) as u64) as u64,
                )),
                Rv32iOpcodeI::Lhu => Ok(x.writeu(
                    rd,
                    bus.load16(x.readi(rs1).wrapping_add(imm as i64) as u64) as u64,
                )),
                Rv32iOpcodeI::Lw => Ok(x.writeu(
                    rd,
                    bus.load32(x.readi(rs1).wrapping_add(imm as i64) as u64) as u64,
                )),
            },
            Instruction::TypeS {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32iOpcodeS::Sb => Ok(bus.store8(
                    x.readi(rs1).wrapping_add(imm as i64) as u64,
                    x.readu(rs2) as u8,
                )),
                Rv32iOpcodeS::Sh => Ok(bus.store16(
                    x.readi(rs1).wrapping_add(imm as i64) as u64,
                    x.readu(rs2) as u16,
                )),
                Rv32iOpcodeS::Sw => Ok(bus.store32(
                    x.readi(rs1).wrapping_add(imm as i64) as u64,
                    x.readu(rs2) as u32,
                )),
            },
            Instruction::TypeB {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32iOpcodeB::Beq => {
                    if x.readu(rs1) == x.readu(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Bne => {
                    if x.readu(rs1) != x.readu(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Blt => {
                    if x.readi(rs1) < x.readi(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Bge => {
                    if x.readi(rs1) >= x.readi(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Bltu => {
                    if x.readu(rs1) < x.readu(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Bgeu => {
                    if x.readu(rs1) >= x.readu(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
            },
            Instruction::TypeU { opcode, rd, imm } => match opcode {
                Rv32iOpcodeU::Lui => Ok(x.writeu(rd, imm)),
                Rv32iOpcodeU::Auipc => Ok(x.writeu(rd, pc.read().wrapping_add(imm))),
            },
            Instruction::TypeJ { opcode, rd, imm } => match opcode {
                Rv32iOpcodeJ::Jal => {
                    x.writeu(rd, pc.read().wrapping_add(4));
                    Ok(pc.jumpr(imm as i64))
                }
            },
        }
    }
}
