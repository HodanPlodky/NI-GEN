enum OpCode {
    Add,
}

enum RegType {
    Void,
    Int,
}

type Register = usize;

struct BasicBlock {
    instruction : Vec<Instruction>,
}

type BBIndex = usize;

enum InstructionType {
    ImmI(i32),
    Reg(Register),
    RegReg(Register, Register),
    RegRegs(Register, Vec<Register>),
    Terminator(BBIndex),
    TerminatorBrach(BBIndex, BBIndex)
}

pub struct Instruction {
    reg_type: RegType,
    name: String,
    op_code : OpCode,
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
