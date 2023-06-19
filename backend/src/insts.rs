pub type Rd = usize;
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
    Call(String),
    Ret,
}

impl AsmInstruction {
    fn size(&self) -> usize {
        match self {
            AsmInstruction::Call(_) => 4,
            AsmInstruction::Add(_, _, _) => 4,
            _ => 4,
        }
    }
}

