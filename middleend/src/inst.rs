use std::ops::{Deref, DerefMut};

use frontend::ast::AstData;

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
    Fun(ImmS),
    Call(RegRegs),
    Arg(ImmI),

    // Terminators
    Ret(Terminator),
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
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RegType {
    Void,
    Int,
    Char,
}

pub type Register = InstUUID;
pub type Symbol = String;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BasicBlock {
    pub instruction: Vec<Instruction>,
}

impl Default for BasicBlock {
    fn default() -> Self {
        Self {
            instruction: vec![],
        }
    }
}

impl Deref for BasicBlock {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.instruction
    }
}

impl DerefMut for BasicBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instruction
    }
}

impl BasicBlock {
    pub fn new(instruction: Vec<Instruction>) -> Self {
        Self { instruction }
    }

    pub fn terminated(&self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.last().unwrap().data.terminator()
        }
    }
}

pub type BBIndex = usize;

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
pub struct RegRegImm(pub Register, pub Register, pub i64);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Terminator;
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerminatorJump(pub BBIndex);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerminatorBranch(pub Register, pub BBIndex, pub BBIndex);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerminatorReg(pub Register);

pub type InstUUID = (bool, usize, usize);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Instruction {
    pub id: InstUUID,
    pub reg_type: RegType,
    pub ast_data: Option<AstData>,
    pub data: InstructionType,
}

impl Instruction {
    pub fn new(
        id: InstUUID,
        reg_type: RegType,
        ast_data: Option<AstData>,
        data: InstructionType,
    ) -> Self {
        Self {
            id,
            reg_type,
            ast_data,
            data,
        }
    }
}

impl From<Instruction> for Register {
    fn from(value: Instruction) -> Self {
        return value.id;
    }
}
