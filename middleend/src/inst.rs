use std::collections::HashMap;

use crate::ir::{BBIndex, Register, Symbol};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InstructionType {
    // basics
    Ldi(ImmI),
    Ldc(ImmC),
    Ld(Reg),
    St(RegReg), // [addr], reg
    Alloca(ImmI),
    Allocg(ImmI),
    Gep(usize, RegRegImm), // [addr], index, offset
    Mov(Reg),

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
    SysCall(ImmIRegs),

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
            InstructionType::Mov(Reg(a)) => vec![*a],
            InstructionType::Gep(_, RegRegImm(a, b, _)) => vec![*a, *b],
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
            InstructionType::SysCall(ImmIRegs(_, regs)) => regs.clone(),
            _ => vec![],
        }
    }

    pub fn rename_regs(&mut self, renames: &HashMap<Register, Register>) {
        match self {
            // unary (Reg)
            InstructionType::Mov(Reg(reg))
            | InstructionType::Branch(TerminatorBranch(reg, _, _))
            | InstructionType::Retr(TerminatorReg(reg))
            | InstructionType::Neg(Reg(reg))
            | InstructionType::Print(Reg(reg))
            | InstructionType::Ld(Reg(reg)) => {
                if renames.contains_key(reg) {
                    *reg = *renames.get(reg).unwrap();
                }
            }

            // binary (RegReg)
            InstructionType::Add(RegReg(a, b))
            | InstructionType::Sub(RegReg(a, b))
            | InstructionType::Mul(RegReg(a, b))
            | InstructionType::Div(RegReg(a, b))
            | InstructionType::Mod(RegReg(a, b))
            | InstructionType::Shr(RegReg(a, b))
            | InstructionType::Shl(RegReg(a, b))
            | InstructionType::And(RegReg(a, b))
            | InstructionType::Or(RegReg(a, b))
            | InstructionType::Xor(RegReg(a, b))
            | InstructionType::Lt(RegReg(a, b))
            | InstructionType::Le(RegReg(a, b))
            | InstructionType::Gt(RegReg(a, b))
            | InstructionType::Ge(RegReg(a, b))
            | InstructionType::Gep(_, RegRegImm(a, b, _))
            | InstructionType::St(RegReg(a, b))
            | InstructionType::Eql(RegReg(a, b)) => {
                if renames.contains_key(a) {
                    *a = *renames.get(a).unwrap();
                }
                if renames.contains_key(b) {
                    *b = *renames.get(b).unwrap();
                }
            }

            // one or multiple regs
            InstructionType::Phi(RegRegs(reg, regs))
            | InstructionType::Call(RegRegs(reg, regs)) => {
                if renames.contains_key(reg) {
                    *reg = *renames.get(reg).unwrap();
                }

                for i in 0..regs.len() {
                    if renames.contains_key(&regs[i]) {
                        regs[i] = *renames.get(&regs[i]).unwrap();
                    }
                }
            }

            InstructionType::CallDirect(SymRegs(_, regs)) => {
                for i in 0..regs.len() {
                    if renames.contains_key(&regs[i]) {
                        regs[i] = *renames.get(&regs[i]).unwrap();
                    }
                }
            }

            InstructionType::SysCall(ImmIRegs(_, regs)) => {
                for i in 0..regs.len() {
                    if renames.contains_key(&regs[i]) {
                        regs[i] = *renames.get(&regs[i]).unwrap();
                    }
                }
            }

            _ => (),
        }
    }
}

// types of instructions
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImmI(pub i64);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImmIRegs(pub i64, pub Vec<Register>);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImmC(pub char);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImmS(pub String);
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
