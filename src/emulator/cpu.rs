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
        csr::{machine_level::*, status::*, supervisor_level::*, user_level::*},
        privileged::{cause::Cause, mode::PrivilegeMode},
    },
};
use std::ops::Range;

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
                let (prv, pc) = handle_trap(
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

fn delegated_privilege_mode(csr: &mut ControlAndStatusRegister, cause: &Cause) -> PrivilegeMode {
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
    if ((csr.csrrs(m_addr, 0) >> code) & 1) == 0 {
        PrivilegeMode::MachineMode
    } else if ((csr.csrrs(s_addr, 0) >> code) & 1) == 0 {
        PrivilegeMode::MachineMode
    } else {
        PrivilegeMode::UserMode
    }
}

fn update_status(
    csr: &mut ControlAndStatusRegister,
    address: u64,
    field: &Range<usize>,
    value: u64,
) {
    let length = (field.end - field.start) as u64;
    let mask = !((2 ^ length) - 1);
    let shifted_value = value << field.start;
    csr.csrrc(address, mask);
    csr.csrrs(address, shifted_value);
}

fn read_status_field(
    csr: &mut ControlAndStatusRegister,
    address: u64,
    field: &Range<usize>,
) -> u64 {
    let status = csr.csrrs(address, 0);
    let length = (field.end - field.start) as u64;
    let mask = !((2 ^ length) - 1);
    (status >> field.start) & mask
}

fn select_address(
    privilege_mode: &PrivilegeMode,
    m_address: u64,
    s_address: u64,
    u_address: u64,
) -> u64 {
    match privilege_mode {
        PrivilegeMode::MachineMode => m_address,
        PrivilegeMode::SupervisorMode => s_address,
        PrivilegeMode::UserMode => u_address,
    }
}

fn select_status_field(
    privilege_mode: &PrivilegeMode,
    m_field: Range<usize>,
    s_field: Range<usize>,
    u_field: Range<usize>,
) -> Range<usize> {
    match privilege_mode {
        PrivilegeMode::MachineMode => m_field,
        PrivilegeMode::SupervisorMode => s_field,
        PrivilegeMode::UserMode => u_field,
    }
}

fn select_tval(cause: &Cause, faulting_address: u64, faulting_instruction: u32) -> u64 {
    match cause {
        Cause::InstructionAddressMisaligned
        | Cause::LoadAddressMisaligned
        | Cause::StoreAddressMisaligned
        | Cause::Breakpoint
        | Cause::InstructionAccessFault
        | Cause::LoadAccessFault
        | Cause::StoreAccessFault
        | Cause::InstructionPageFault
        | Cause::LoadPageFault
        | Cause::StorePageFault => faulting_address,
        Cause::IllegalInstruction => faulting_instruction as u64,
        _ => 0,
    }
}

fn handle_trap(
    cause: &Cause,
    pc_address: u64,
    instruction: u32,
    current_privilege_mode: PrivilegeMode,
    csr: &mut ControlAndStatusRegister,
) -> (PrivilegeMode, u64) {
    let privilege_mode = delegated_privilege_mode(csr, cause);
    // set cause register
    let cause_address = select_address(&privilege_mode, MCAUSE, SCAUSE, UCAUSE);
    csr.csrrw(cause_address, cause.to_primitive());

    // set exception program counter
    let epc_address = select_address(&privilege_mode, MEPC, SEPC, UEPC);
    csr.csrrw(epc_address, pc_address);

    // set trap value register
    let tval_address = select_address(&privilege_mode, MTVAL, STVAL, UTVAL);
    let tval = select_tval(&cause, pc_address, instruction);
    csr.csrrw(tval_address, tval);

    // set previous privilege
    let status_address = select_address(&privilege_mode, MSTATUS, SSTATUS, USTATUS);
    match privilege_mode {
        PrivilegeMode::MachineMode => update_status(
            csr,
            status_address,
            &STATUS_MPP,
            current_privilege_mode as u64,
        ),
        PrivilegeMode::SupervisorMode => update_status(
            csr,
            status_address,
            &STATUS_SPP,
            current_privilege_mode as u64,
        ),
        PrivilegeMode::UserMode => {}
    }

    // set previous interrupt enable
    let ie_field = select_status_field(&privilege_mode, STATUS_MIE, STATUS_SIE, STATUS_UIE);
    let ie = read_status_field(csr, status_address, &ie_field);
    let pie_field = select_status_field(&privilege_mode, STATUS_MPIE, STATUS_SPIE, STATUS_UPIE);
    update_status(csr, status_address, &pie_field, ie);

    // disable interrupt enable
    update_status(csr, status_address, &ie_field, 0);

    // set pc to trap-vector base-address register
    let tvec_address = select_address(&privilege_mode, MTVEC, STVEC, UTVEC);
    let tvec = csr.csrrs(tvec_address, 0);
    (privilege_mode, tvec)

    // ここからRET後の処理
    // restore interrupt enable
    // let pie = self.read_status_field(csr, status_address, &pie_field);
    // self.update_status(csr, status_address, &ie_field, pie);

    // 15. U-modeに遷移する
    // 16. sstatus.SPIE <~ 1 [always]
    // 17. sstatus.SPP <~ 00(U-mode) [always]
    // 18. pc(program counter) <~ sepc CSR
}
