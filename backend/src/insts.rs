use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Rd {
    // Pseudo registers
    Ir(middleend::ir::Register),
    ArgReg(u8),
    Zero,
    Sp,
    Ra,

    // Real architectural registers
    Arch(usize),
}

impl Display for Rd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rd::Ir(reg) => write!(f, "ir=({})", reg.val()),
            Rd::ArgReg(number) => write!(f, "a{}", number),
            Rd::Zero => write!(f, "zero"),
            Rd::Sp => write!(f, "sp"),
            Rd::Ra => write!(f, "ra"),
            Rd::Arch(number) => write!(f, "x{}", number),
        }
    }
}

pub type Imm = i64;
pub type Offset = i64;

#[derive(Clone, PartialEq, Eq)]
pub enum AsmInstruction {
    Lui(Rd, Imm),
    Auipc(Rd, Imm),

    Jal(Rd, Offset, String),
    Jalr(Rd, Rd, Offset),

    Beq(Rd, Rd, Offset, String),
    Bne(Rd, Rd, Offset, String),
    Blt(Rd, Rd, Offset, String),
    Bge(Rd, Rd, Offset, String),
    Bltu(Rd, Rd, Offset, String),
    Bgeu(Rd, Rd, Offset, String),

    Lb(Rd, Rd, Offset),
    Lh(Rd, Rd, Offset),
    Lw(Rd, Rd, Offset),
    Ld(Rd, Rd, Offset),
    Lbu(Rd, Rd, Offset),
    Lhu(Rd, Rd, Offset),

    Sb(Rd, Rd, Offset),
    Sh(Rd, Rd, Offset),
    Sw(Rd, Rd, Offset),
    Sd(Rd, Rd, Offset),

    Addi(Rd, Rd, Imm),
    Slti(Rd, Rd, Imm),
    Sltiu(Rd, Rd, Imm),
    Xori(Rd, Rd, Imm),
    Ori(Rd, Rd, Imm),
    Andi(Rd, Rd, Imm),
    Slli(Rd, Rd, Imm),
    Srli(Rd, Rd, Imm),
    Srai(Rd, Rd, Imm),

    Add(Rd, Rd, Rd),
    Mul(Rd, Rd, Rd),
    Sub(Rd, Rd, Rd),
    Sll(Rd, Rd, Rd),
    Srl(Rd, Rd, Rd),
    Slt(Rd, Rd, Rd),
    Sltu(Rd, Rd, Rd),
    Xor(Rd, Rd, Rd),
    Or(Rd, Rd, Rd),
    And(Rd, Rd, Rd),
    Sra(Rd, Rd, Rd),

    // pseudo instructions
    Call(String, middleend::ir::InstUUID),
    Ret,

    Ecall,
}

impl AsmInstruction {
    #[allow(dead_code)]
    fn size(&self) -> usize {
        match self {
            AsmInstruction::Call(_, _) => 4,
            AsmInstruction::Add(_, _, _) => 4,
            _ => 4,
        }
    }

    pub fn get_reads(&self) -> Vec<Rd> {
        match self {
            AsmInstruction::Lui(_, _) => todo!(),
            AsmInstruction::Auipc(_, _) => todo!(),
            &AsmInstruction::Jalr(_, rs1, _) => vec![rs1],
            &AsmInstruction::Beq(rs1, rs2, _, _) => vec![rs1, rs2],
            &AsmInstruction::Bne(rs1, rs2, _, _) => vec![rs1, rs2],
            &AsmInstruction::Blt(rs1, rs2, _, _) => vec![rs1, rs2],
            &AsmInstruction::Bge(rs1, rs2, _, _) => vec![rs1, rs2],
            &AsmInstruction::Bltu(rs1, rs2, _, _) => vec![rs1, rs2],
            &AsmInstruction::Bgeu(rs1, rs2, _, _) => vec![rs1, rs2],
            &AsmInstruction::Lb(_, rs1, _) => vec![rs1],
            &AsmInstruction::Lh(_, rs1, _) => vec![rs1],
            &AsmInstruction::Lw(_, rs1, _) => vec![rs1],
            &AsmInstruction::Ld(_, rs1, _) => vec![rs1],
            &AsmInstruction::Lbu(_, rs1, _) => vec![rs1],
            &AsmInstruction::Lhu(_, rs1, _) => vec![rs1],
            &AsmInstruction::Sb(rs1, rs2, _) => vec![rs1, rs2],
            &AsmInstruction::Sh(rs1, rs2, _) => vec![rs1, rs2],
            &AsmInstruction::Sw(rs1, rs2, _) => vec![rs1, rs2],
            &AsmInstruction::Sd(rs1, rs2, _) => vec![rs1, rs2],
            &AsmInstruction::Addi(_, rs1, _) => vec![rs1],
            &AsmInstruction::Slti(_, rs1, _) => vec![rs1],
            &AsmInstruction::Sltiu(_, rs1, _) => vec![rs1],
            &AsmInstruction::Xori(_, rs1, _) => vec![rs1],
            &AsmInstruction::Ori(_, rs1, _) => vec![rs1],
            &AsmInstruction::Andi(_, rs1, _) => vec![rs1],
            &AsmInstruction::Slli(_, rs1, _) => vec![rs1],
            &AsmInstruction::Srli(_, rs1, _) => vec![rs1],
            &AsmInstruction::Srai(_, rs1, _) => vec![rs1],
            &AsmInstruction::Add(_, rs1, rs2) => vec![rs1, rs2],
            &AsmInstruction::Mul(_, rs1, rs2) => vec![rs1, rs2],
            &AsmInstruction::Sub(_, rs1, rs2) => vec![rs1, rs2],
            &AsmInstruction::Sll(_, rs1, rs2) => vec![rs1, rs2],
            &AsmInstruction::Srl(_, rs1, rs2) => vec![rs1, rs2],
            &AsmInstruction::Slt(_, rs1, rs2) => vec![rs1, rs2],
            &AsmInstruction::Sltu(_, rs1, rs2) => vec![rs1, rs2],
            &AsmInstruction::Xor(_, rs1, rs2) => vec![rs1, rs2],
            &AsmInstruction::Or(_, rs1, rs2) => vec![rs1, rs2],
            &AsmInstruction::And(_, rs1, rs2) => vec![rs1, rs2],
            &AsmInstruction::Sra(_, rs1, rs2) => vec![rs1, rs2],
            _ => vec![],
        }
    }

    pub fn get_write(&self) -> Option<Rd> {
        match self {
            &AsmInstruction::Lui(_, _) => todo!(),
            &AsmInstruction::Auipc(_, _) => todo!(),
            &AsmInstruction::Jal(rd, _, _) => Some(rd),
            &AsmInstruction::Jalr(rd, _, _) => Some(rd),
            &AsmInstruction::Lb(rd, _, _) => Some(rd),
            &AsmInstruction::Lh(rd, _, _) => Some(rd),
            &AsmInstruction::Lw(rd, _, _) => Some(rd),
            &AsmInstruction::Ld(rd, _, _) => Some(rd),
            &AsmInstruction::Lbu(rd, _, _) => Some(rd),
            &AsmInstruction::Lhu(rd, _, _) => Some(rd),
            &AsmInstruction::Addi(rd, _, _) => Some(rd),
            &AsmInstruction::Slti(rd, _, _) => Some(rd),
            &AsmInstruction::Sltiu(rd, _, _) => Some(rd),
            &AsmInstruction::Xori(rd, _, _) => Some(rd),
            &AsmInstruction::Ori(rd, _, _) => Some(rd),
            &AsmInstruction::Andi(rd, _, _) => Some(rd),
            &AsmInstruction::Slli(rd, _, _) => Some(rd),
            &AsmInstruction::Srli(rd, _, _) => Some(rd),
            &AsmInstruction::Srai(rd, _, _) => Some(rd),
            &AsmInstruction::Add(rd, _, _) => Some(rd),
            &AsmInstruction::Mul(rd, _, _) => Some(rd),
            &AsmInstruction::Sub(rd, _, _) => Some(rd),
            &AsmInstruction::Sll(rd, _, _) => Some(rd),
            &AsmInstruction::Srl(rd, _, _) => Some(rd),
            &AsmInstruction::Slt(rd, _, _) => Some(rd),
            &AsmInstruction::Sltu(rd, _, _) => Some(rd),
            &AsmInstruction::Xor(rd, _, _) => Some(rd),
            &AsmInstruction::Or(rd, _, _) => Some(rd),
            &AsmInstruction::And(rd, _, _) => Some(rd),
            &AsmInstruction::Sra(rd, _, _) => Some(rd),
            _ => None,
        }
    }
}
