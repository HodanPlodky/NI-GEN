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
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RegType {
    Void,
    Int,
    Char,
}

pub type Register = InstUUID;
pub type Symbol = String;

pub struct BasicBlock {
    pub instruction: Vec<Instruction>,
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
pub struct TerminatorBranch(pub BBIndex, pub BBIndex);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerminatorReg(pub Reg, pub BBIndex);

pub type InstUUID = usize;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Instruction {
    pub id: InstUUID,
    pub reg_type: RegType,
    pub ast_data: AstData,
    pub data: InstructionType,
}

impl From<Instruction> for Register {
    fn from(value: Instruction) -> Self {
        return value.id;
    }
}
