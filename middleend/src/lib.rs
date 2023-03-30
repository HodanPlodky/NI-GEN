pub enum OpCode {
    Ldi,
    Ld,
    Add,
}

impl From<OpCode> for u8 {
    fn from(_: OpCode) -> Self {
        todo!()
    }
}

impl From<u8> for OpCode {
    fn from(_: u8) -> Self {
        todo!()
    }
}

pub enum RegType {
    Void,
    Int,
}

pub type Register = usize;

pub struct BasicBlock {
    instruction: Vec<Instruction>,
}

pub type BBIndex = usize;

pub enum InstructionType {
    ImmI(i32),
    Reg(Register),
    RegReg(Register, Register),
    RegRegs(Register, Vec<Register>),
    Terminator(BBIndex),
    TerminatorBrach(BBIndex, BBIndex),
}

pub struct Instruction {
    reg_type: RegType,
    name: String,
    op_code: OpCode,
    data: InstructionType,
}

impl Instruction {
    pub fn new(reg_type: RegType, name: &str, op_code: OpCode, data: InstructionType) -> Self {
        Self {
            reg_type,
            name: name.to_string(),
            op_code,
            data,
        }
    }

    pub fn ldi(num: i32) -> Self {
        Self::new(RegType::Int, "ldi", OpCode::Add, InstructionType::ImmI(num))
    }

    pub fn ld(reg : Register) -> Self {
        Self::new(RegType::Int, "ld", OpCode::Ldi, InstructionType::Reg(reg))
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
