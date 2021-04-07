mod csr;
mod decoder;
mod executor;
mod f;
mod pc;
mod trap_handler;
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
            trap_handler::*,
            x::{IntegerRegister, A0},
        },
    },
    isa::privileged::mode::PrivilegeMode,
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
            let result = if let Some(decoded) = PrivilegedDecoder::decode(instruction) {
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

            // handle the trap
            if let Err(cause) = result {
                let (prv, pc) = handle_cause(
                    &cause,
                    self.pc.read(),
                    instruction,
                    self.privilege_mode,
                    &mut self.csr,
                );
                self.privilege_mode = prv;
                self.pc.jump(pc);
            }
            // increment the pc when the pc has not been updated
            else if self.pc.read() == address {
                self.pc.increment();
            }
        }
        self.x.readu(A0)
    }
}
