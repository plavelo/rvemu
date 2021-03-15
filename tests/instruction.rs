use five::emulator::Emulator;
use std::fs::File;
use std::path::PathBuf;

fn run(name: &str) -> bool {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("riscv-tests");
    path.push("isa");
    path.push(name);
    path.set_extension("bin");
    let file = File::open(&path.as_path());
    let mut emulator = Emulator::default();
    if let Ok(f) = file {
        let _ = emulator.load(f);
        emulator.run() == 1
    } else {
        false
    }
}

#[test]
fn rv32ui_p_ok() {
    assert!(run("rv32ui-p-add"), "{}", "rv32ui-p-add");
    assert!(run("rv32ui-p-addi"), "{}", "rv32ui-p-addi");
    assert!(run("rv32ui-p-and"), "{}", "rv32ui-p-and");
    assert!(run("rv32ui-p-andi"), "{}", "rv32ui-p-andi");
    assert!(run("rv32ui-p-auipc"), "{}", "rv32ui-p-auipc");
    assert!(run("rv32ui-p-beq"), "{}", "rv32ui-p-beq");
    assert!(run("rv32ui-p-bge"), "{}", "rv32ui-p-bge");
    assert!(run("rv32ui-p-bgeu"), "{}", "rv32ui-p-bgeu");
    assert!(run("rv32ui-p-blt"), "{}", "rv32ui-p-blt");
    assert!(run("rv32ui-p-bltu"), "{}", "rv32ui-p-bltu");
    assert!(run("rv32ui-p-bne"), "{}", "rv32ui-p-bne");
    assert!(run("rv32ui-p-fence_i"), "{}", "rv32ui-p-fence_i");
    assert!(run("rv32ui-p-jal"), "{}", "rv32ui-p-jal");
    assert!(run("rv32ui-p-jalr"), "{}", "rv32ui-p-jalr");
    assert!(run("rv32ui-p-lb"), "{}", "rv32ui-p-lb");
    assert!(run("rv32ui-p-lbu"), "{}", "rv32ui-p-lbu");
    assert!(run("rv32ui-p-lh"), "{}", "rv32ui-p-lh");
    assert!(run("rv32ui-p-lhu"), "{}", "rv32ui-p-lhu");
    assert!(run("rv32ui-p-lui"), "{}", "rv32ui-p-lui");
    assert!(run("rv32ui-p-lw"), "{}", "rv32ui-p-lw");
    assert!(run("rv32ui-p-or"), "{}", "rv32ui-p-or");
    assert!(run("rv32ui-p-ori"), "{}", "rv32ui-p-ori");
    assert!(run("rv32ui-p-sb"), "{}", "rv32ui-p-sb");
    assert!(run("rv32ui-p-sh"), "{}", "rv32ui-p-sh");
    assert!(run("rv32ui-p-simple"), "{}", "rv32ui-p-simple");
    assert!(run("rv32ui-p-sll"), "{}", "rv32ui-p-sll");
    assert!(run("rv32ui-p-slli"), "{}", "rv32ui-p-slli");
    assert!(run("rv32ui-p-slt"), "{}", "rv32ui-p-slt");
    assert!(run("rv32ui-p-slti"), "{}", "rv32ui-p-slti");
    assert!(run("rv32ui-p-sltiu"), "{}", "rv32ui-p-sltiu");
    assert!(run("rv32ui-p-sltu"), "{}", "rv32ui-p-sltu");
    assert!(run("rv32ui-p-sra"), "{}", "rv32ui-p-sra");
    assert!(run("rv32ui-p-srai"), "{}", "rv32ui-p-srai");
    assert!(run("rv32ui-p-srl"), "{}", "rv32ui-p-srl");
    assert!(run("rv32ui-p-srli"), "{}", "rv32ui-p-srli");
    assert!(run("rv32ui-p-sub"), "{}", "rv32ui-p-sub");
    assert!(run("rv32ui-p-sw"), "{}", "rv32ui-p-sw");
    assert!(run("rv32ui-p-xor"), "{}", "rv32ui-p-xor");
    assert!(run("rv32ui-p-xori"), "{}", "rv32ui-p-xori");
}

#[test]
fn rv64ui_p_ok() {
    assert!(run("rv64ui-p-add"), "{}", "rv64ui-p-add");
    assert!(run("rv64ui-p-addi"), "{}", "rv64ui-p-addi");
    assert!(run("rv64ui-p-addiw"), "{}", "rv64ui-p-addiw");
    assert!(run("rv64ui-p-addw"), "{}", "rv64ui-p-addw");
    assert!(run("rv64ui-p-and"), "{}", "rv64ui-p-and");
    assert!(run("rv64ui-p-andi"), "{}", "rv64ui-p-andi");
    assert!(run("rv64ui-p-auipc"), "{}", "rv64ui-p-auipc");
    assert!(run("rv64ui-p-beq"), "{}", "rv64ui-p-beq");
    assert!(run("rv64ui-p-bge"), "{}", "rv64ui-p-bge");
    assert!(run("rv64ui-p-bgeu"), "{}", "rv64ui-p-bgeu");
    assert!(run("rv64ui-p-blt"), "{}", "rv64ui-p-blt");
    assert!(run("rv64ui-p-bltu"), "{}", "rv64ui-p-bltu");
    assert!(run("rv64ui-p-bne"), "{}", "rv64ui-p-bne");
    assert!(run("rv64ui-p-fence_i"), "{}", "rv64ui-p-fence_i");
    assert!(run("rv64ui-p-jal"), "{}", "rv64ui-p-jal");
    assert!(run("rv64ui-p-jalr"), "{}", "rv64ui-p-jalr");
    assert!(run("rv64ui-p-lb"), "{}", "rv64ui-p-lb");
    assert!(run("rv64ui-p-lbu"), "{}", "rv64ui-p-lbu");
    assert!(run("rv64ui-p-ld"), "{}", "rv64ui-p-ld");
    assert!(run("rv64ui-p-lh"), "{}", "rv64ui-p-lh");
    assert!(run("rv64ui-p-lhu"), "{}", "rv64ui-p-lhu");
    assert!(run("rv64ui-p-lui"), "{}", "rv64ui-p-lui");
    assert!(run("rv64ui-p-lw"), "{}", "rv64ui-p-lw");
    assert!(run("rv64ui-p-lwu"), "{}", "rv64ui-p-lwu");
    assert!(run("rv64ui-p-or"), "{}", "rv64ui-p-or");
    assert!(run("rv64ui-p-ori"), "{}", "rv64ui-p-ori");
    assert!(run("rv64ui-p-sb"), "{}", "rv64ui-p-sb");
    assert!(run("rv64ui-p-sd"), "{}", "rv64ui-p-sd");
    assert!(run("rv64ui-p-sh"), "{}", "rv64ui-p-sh");
    assert!(run("rv64ui-p-simple"), "{}", "rv64ui-p-simple");
    assert!(run("rv64ui-p-sll"), "{}", "rv64ui-p-sll");
    assert!(run("rv64ui-p-slli"), "{}", "rv64ui-p-slli");
    assert!(run("rv64ui-p-slliw"), "{}", "rv64ui-p-slliw");
    assert!(run("rv64ui-p-sllw"), "{}", "rv64ui-p-sllw");
    assert!(run("rv64ui-p-slt"), "{}", "rv64ui-p-slt");
    assert!(run("rv64ui-p-slti"), "{}", "rv64ui-p-slti");
    assert!(run("rv64ui-p-sltiu"), "{}", "rv64ui-p-sltiu");
    assert!(run("rv64ui-p-sltu"), "{}", "rv64ui-p-sltu");
    assert!(run("rv64ui-p-sra"), "{}", "rv64ui-p-sra");
    assert!(run("rv64ui-p-srai"), "{}", "rv64ui-p-srai");
    assert!(run("rv64ui-p-sraiw"), "{}", "rv64ui-p-sraiw");
    assert!(run("rv64ui-p-sraw"), "{}", "rv64ui-p-sraw");
    assert!(run("rv64ui-p-srl"), "{}", "rv64ui-p-srl");
    assert!(run("rv64ui-p-srli"), "{}", "rv64ui-p-srli");
    assert!(run("rv64ui-p-srliw"), "{}", "rv64ui-p-srliw");
    assert!(run("rv64ui-p-srlw"), "{}", "rv64ui-p-srlw");
    assert!(run("rv64ui-p-sub"), "{}", "rv64ui-p-sub");
    assert!(run("rv64ui-p-subw"), "{}", "rv64ui-p-subw");
    assert!(run("rv64ui-p-sw"), "{}", "rv64ui-p-sw");
    assert!(run("rv64ui-p-xor"), "{}", "rv64ui-p-xor");
    assert!(run("rv64ui-p-xori"), "{}", "rv64ui-p-xori");
}

#[test]
fn rv32um_p_ok() {
    assert!(run("rv32um-p-div"), "{}", "rv32um-p-div");
    assert!(run("rv32um-p-divu"), "{}", "rv32um-p-divu");
    assert!(run("rv32um-p-mul"), "{}", "rv32um-p-mul");
    assert!(run("rv32um-p-mulh"), "{}", "rv32um-p-mulh");
    assert!(run("rv32um-p-mulhsu"), "{}", "rv32um-p-mulhsu");
    assert!(run("rv32um-p-mulhu"), "{}", "rv32um-p-mulhu");
    assert!(run("rv32um-p-rem"), "{}", "rv32um-p-rem");
    assert!(run("rv32um-p-remu"), "{}", "rv32um-p-remu");
}

#[test]
fn rv64um_p_ok() {
    assert!(run("rv64um-p-div"), "{}", "rv64um-p-div");
    assert!(run("rv64um-p-divu"), "{}", "rv64um-p-divu");
    assert!(run("rv64um-p-divuw"), "{}", "rv64um-p-divuw");
    assert!(run("rv64um-p-divw"), "{}", "rv64um-p-divw");
    assert!(run("rv64um-p-mul"), "{}", "rv64um-p-mul");
    assert!(run("rv64um-p-mulh"), "{}", "rv64um-p-mulh");
    assert!(run("rv64um-p-mulhsu"), "{}", "rv64um-p-mulhsu");
    assert!(run("rv64um-p-mulhu"), "{}", "rv64um-p-mulhu");
    assert!(run("rv64um-p-mulw"), "{}", "rv64um-p-mulw");
    assert!(run("rv64um-p-rem"), "{}", "rv64um-p-rem");
    assert!(run("rv64um-p-remu"), "{}", "rv64um-p-remu");
    assert!(run("rv64um-p-remuw"), "{}", "rv64um-p-remuw");
    assert!(run("rv64um-p-remw"), "{}", "rv64um-p-remw");
}

#[test]
fn rv32uf_p_ok() {
    assert!(run("rv32uf-p-fadd"), "{}", "rv32uf-p-fadd");
    assert!(run("rv32uf-p-fclass"), "{}", "rv32uf-p-fclass");
    assert!(run("rv32uf-p-fcmp"), "{}", "rv32uf-p-fcmp");
    assert!(run("rv32uf-p-fcvt"), "{}", "rv32uf-p-fcvt");
    assert!(run("rv32uf-p-fcvt_w"), "{}", "rv32uf-p-fcvt_w");
    assert!(run("rv32uf-p-fdiv"), "{}", "rv32uf-p-fdiv");
    assert!(run("rv32uf-p-fmadd"), "{}", "rv32uf-p-fmadd");
    assert!(run("rv32uf-p-fmin"), "{}", "rv32uf-p-fmin");
    assert!(run("rv32uf-p-ldst"), "{}", "rv32uf-p-ldst");
    assert!(run("rv32uf-p-move"), "{}", "rv32uf-p-move");
    assert!(run("rv32uf-p-recoding"), "{}", "rv32uf-p-recoding");
}
