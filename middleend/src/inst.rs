use crate::ir::{Symbol, Register, BBIndex};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InstructionType {
    // basics
    Ldi(ImmI),
    Ldc(ImmC),
    Ld(Reg),
    St(RegReg),
    Alloca(ImmI),
    Allocg(ImmI),
    Cpy(RegRegImm),
    Gep(RegRegImm),

    // number binary
    Add(RegReg),
    Sub(RegReg),
    Mul(RegReg),
    Div(RegReg),
    Mod(RegReg),
    Shr(RegReg),
    Shl(RegReg),

    // bitwise binary
    And(RegReg),
    Or(RegReg),
    Xor(RegReg),

    // bitwise unary
    Neg(Reg),

    // comparion binary
    Lt(RegReg),
    Le(RegReg),
    Gt(RegReg),
    Ge(RegReg),
    Eql(RegReg),

    // functions
    Call(RegRegs),
    CallDirect(SymRegs),
    Arg(ImmI),

    // Terminators
    Ret(Terminator),
    Exit(Terminator),
    Retr(TerminatorReg),
    Jmp(TerminatorJump),
    Branch(TerminatorBranch),

    // instrisic
    Print(Reg),

    // phi node
    Phi(RegRegs),
}

impl InstructionType {
    pub fn terminator(&self) -> bool {
        match self {
            InstructionType::Ret(_)
            | InstructionType::Retr(_)
            | InstructionType::Jmp(_)
            | InstructionType::Branch(_) => true,
            _ => false,
        }
    }

    pub fn get_regs(&self) -> Vec<Register> {
        match self {
            InstructionType::Ld(Reg(a)) => vec![*a],
            InstructionType::St(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Cpy(RegRegImm(a, b, _)) => vec![*a, *b],
            InstructionType::Gep(RegRegImm(a, b, _)) => vec![*a, *b],
            InstructionType::Add(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Sub(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Mul(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Div(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Mod(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Shr(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Shl(RegReg(a, b)) => vec![*a, *b],
            InstructionType::And(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Or(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Xor(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Neg(Reg(a)) => vec![*a],
            InstructionType::Lt(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Le(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Gt(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Ge(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Eql(RegReg(a, b)) => vec![*a, *b],
            InstructionType::Call(RegRegs(reg, regs)) => {
                let mut regs = regs.clone();
                regs.push(*reg);
                regs
            }
            InstructionType::CallDirect(SymRegs(_, regs)) => regs.clone(),
            InstructionType::Retr(TerminatorReg(reg)) => vec![*reg],
            InstructionType::Branch(TerminatorBranch(reg, _, _)) => vec![*reg],
            InstructionType::Print(Reg(a)) => vec![*a],
            InstructionType::Phi(RegRegs(reg, regs)) => {
                let mut regs = regs.clone();
                regs.push(*reg);
                regs
            }
            _ => vec![],
        }
    }
}


// types of instructions
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImmI(pub i64);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImmC(pub char);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImmS(pub Symbol);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Reg(pub Register);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RegReg(pub Register, pub Register);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RegRegs(pub Register, pub Vec<Register>);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SymRegs(pub Symbol, pub Vec<Register>);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RegRegImm(pub Register, pub Register, pub i64);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Terminator;
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerminatorJump(pub BBIndex);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerminatorBranch(pub Register, pub BBIndex, pub BBIndex);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerminatorReg(pub Register);

