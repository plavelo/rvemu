mod csr;
mod decoder;
mod executor;
mod f;
mod pc;
mod x;

use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister,
            decoder::{
                privileged::PrivilegedDecoder, rv32f::Rv32fDecoder, rv32i::Rv32iDecoder,
                rv32m::Rv32mDecoder, rv64i::Rv64iDecoder, rv64m::Rv64mDecoder, zicsr::ZicsrDecoder,
                zifencei::ZifenceiDecoder, Decoder,
            },
            executor::{
                privileged::PrivilegedExecutor, rv32f::Rv32fExecutor, rv32i::Rv32iExecutor,
                rv32m::Rv32mExecutor, rv64i::Rv64iExecutor, rv64m::Rv64mExecutor,
                zicsr::ZicsrExecutor, zifencei::ZifenceiExecutor, Executor,
            },
            f::FloatingPointRegister,
            pc::ProgramCounter,
            x::{IntegerRegister, A0},
        },
    },
    isa::{
        csr::{machine_level::*, supervisor_level::*},
        privileged::{cause::Cause, mode::PrivilegeMode},
    },
};

#[derive(Default)]
pub struct Cpu {
    x: IntegerRegister,
    f: FloatingPointRegister,
    pc: ProgramCounter,
    csr: ControlAndStatusRegister,
    privilege_mode: PrivilegeMode,
    pub bus: SystemBus,
}

impl Cpu {
    pub fn run(&mut self) -> u64 {
        while self.pc.read() < self.bus.memory.size() {
            // read an address from the pc
            let address = self.pc.read();
            // fetch an instruction
            let instruction = self.bus.load32(address);
            // decode and execute the instruction
            if let Some(decoded) = PrivilegedDecoder::decode(instruction) {
                PrivilegedExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = ZifenceiDecoder::decode(instruction) {
                ZifenceiExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = ZicsrDecoder::decode(instruction) {
                ZicsrExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv32iDecoder::decode(instruction) {
                Rv32iExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv64iDecoder::decode(instruction) {
                Rv64iExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv32mDecoder::decode(instruction) {
                Rv32mExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv64mDecoder::decode(instruction) {
                Rv64mExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv32fDecoder::decode(instruction) {
                Rv32fExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else {
                // end the loop when unable to decode the instruction
                break;
            };
            // increment the pc when the pc has not been updated
            if self.pc.read() == address {
                self.pc.increment();
            }
        }
        self.x.readu(A0)
    }

    fn delegated_privilege_mode(&self, cause: Cause) -> PrivilegeMode {
        let m_addr = if cause.is_interrupt() {
            MIDELEG
        } else {
            MEDELEG
        };
        let s_addr = if cause.is_interrupt() {
            SIDELEG
        } else {
            SEDELEG
        };
        let code = cause.exception_code();
        if ((self.csr.csrrs(m_addr, 0) >> code) & 1) == 0 {
            PrivilegeMode::MachineMode
        } else if ((self.csr.csrrs(s_addr, 0) >> code) & 1) == 0 {
            PrivilegeMode::MachineMode
        } else {
            PrivilegeMode::UserMode
        }
    }

    fn handle_trap(&self) {
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
    }
}
