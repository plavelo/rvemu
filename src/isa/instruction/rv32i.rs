use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeR {
    Sll,
    Srl,
    Sra,
    Add,
    Sub,
    Xor,
    Or,
    And,
    Slt,
    Sltu,
}

impl fmt::Display for Rv32iOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Sll => f.write_str("Rv32i::Sll"),
            Self::Srl => f.write_str("Rv32i::Srl"),
            Self::Sra => f.write_str("Rv32i::Sra"),
            Self::Add => f.write_str("Rv32i::Add"),
            Self::Sub => f.write_str("Rv32i::Sub"),
            Self::Xor => f.write_str("Rv32i::Xor"),
            Self::Or => f.write_str("Rv32i::Or"),
            Self::And => f.write_str("Rv32i::And"),
            Self::Slt => f.write_str("Rv32i::Slt"),
            Self::Sltu => f.write_str("Rv32i::Sltu"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeI {
    Slli,
    Srli,
    Srai,
    Addi,
    Xori,
    Ori,
    Andi,
    Slti,
    Sltiu,
    Jalr,
    Fence,
    Ecall,
    Ebreak,
    Lb,
    Lh,
    Lbu,
    Lhu,
    Lw,
}

impl fmt::Display for Rv32iOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Slli => f.write_str("Rv32i::Slli"),
            Self::Srli => f.write_str("Rv32i::Srli"),
            Self::Srai => f.write_str("Rv32i::Srai"),
            Self::Addi => f.write_str("Rv32i::Addi"),
            Self::Xori => f.write_str("Rv32i::Xori"),
            Self::Ori => f.write_str("Rv32i::Ori"),
            Self::Andi => f.write_str("Rv32i::Andi"),
            Self::Slti => f.write_str("Rv32i::Slti"),
            Self::Sltiu => f.write_str("Rv32i::Sltiu"),
            Self::Jalr => f.write_str("Rv32i::Jalr"),
            Self::Fence => f.write_str("Rv32i::Fence"),
            Self::Ecall => f.write_str("Rv32i::Ecall"),
            Self::Ebreak => f.write_str("Rv32i::Ebreak"),
            Self::Lb => f.write_str("Rv32i::Lb"),
            Self::Lh => f.write_str("Rv32i::Lh"),
            Self::Lbu => f.write_str("Rv32i::Lbu"),
            Self::Lhu => f.write_str("Rv32i::Lhu"),
            Self::Lw => f.write_str("Rv32i::Lw"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeS {
    Sb,
    Sh,
    Sw,
}

impl fmt::Display for Rv32iOpcodeS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Sb => f.write_str("Rv32i::Sb"),
            Self::Sh => f.write_str("Rv32i::Sh"),
            Self::Sw => f.write_str("Rv32i::Sw"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeB {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

impl fmt::Display for Rv32iOpcodeB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Beq => f.write_str("Rv32i::Beq"),
            Self::Bne => f.write_str("Rv32i::Bne"),
            Self::Blt => f.write_str("Rv32i::Blt"),
            Self::Bge => f.write_str("Rv32i::Bge"),
            Self::Bltu => f.write_str("Rv32i::Bltu"),
            Self::Bgeu => f.write_str("Rv32i::Bgeu"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeU {
    Lui,
    Auipc,
}

impl fmt::Display for Rv32iOpcodeU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Lui => f.write_str("Rv32i::Lui"),
            Self::Auipc => f.write_str("Rv32i::Auipc"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeJ {
    Jal,
}

impl fmt::Display for Rv32iOpcodeJ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Jal => f.write_str("Rv32i::Jal"),
        }
    }
}
