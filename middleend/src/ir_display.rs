use std::fmt::Display;

use crate::{
    inst::{BasicBlock, Instruction, InstructionType, RegType, Register},
    ir::{Function, IrProgram},
};

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionType::Ldi(_) => todo!(),
            InstructionType::Ldc(_) => todo!(),
            InstructionType::Ld(_) => todo!(),
            InstructionType::St(_) => todo!(),
            InstructionType::Alloca(_) => todo!(),
            InstructionType::Allocg(_) => todo!(),
            InstructionType::Cpy(_) => todo!(),
            InstructionType::Gep(_) => todo!(),
            InstructionType::Add(_) => todo!(),
            InstructionType::Sub(_) => todo!(),
            InstructionType::Mul(_) => todo!(),
            InstructionType::Div(_) => todo!(),
            InstructionType::Mod(_) => todo!(),
            InstructionType::Shr(_) => todo!(),
            InstructionType::Shl(_) => todo!(),
            InstructionType::And(_) => todo!(),
            InstructionType::Or(_) => todo!(),
            InstructionType::Xor(_) => todo!(),
            InstructionType::Neg(_) => todo!(),
            InstructionType::Lt(_) => todo!(),
            InstructionType::Le(_) => todo!(),
            InstructionType::Gt(_) => todo!(),
            InstructionType::Ge(_) => todo!(),
            InstructionType::Eql(_) => todo!(),
            InstructionType::Fun(_) => todo!(),
            InstructionType::Call(_) => todo!(),
            InstructionType::Arg(_) => todo!(),
            InstructionType::Ret(_) => write!(f, "ret"),
            InstructionType::Retr(_) => todo!(),
            InstructionType::Jmp(_) => todo!(),
            InstructionType::Branch(_) => todo!(),
            InstructionType::Print(_) => todo!(),
        }
    }
}

impl Display for RegType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegType::Void => write!(f, "void"),
            RegType::Int => write!(f, "int"),
            RegType::Char => write!(f, "char"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg_string = if self.id.0 {
            "g(".to_string()
                + self.id.1.to_string().as_str()
                + ","
                + self.id.2.to_string().as_str()
                + ")"
        } else {
            "(".to_string()
                + self.id.1.to_string().as_str()
                + ","
                + self.id.2.to_string().as_str()
                + ")"
        };
        write!(f, "{} : {} = {}", reg_string, self.reg_type, self.data)
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.instruction.iter() {
            writeln!(f, "\t{}", i)?;
        }
        Ok(())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "function {}({}) : {} {{",
            self.name, self.arg_count, self.ret_type
        )?;
        for i in 0..self.blocks.len() {
            writeln!(f, "BB{}:", i)?;
            writeln!(f, "\t{}", self.blocks[i])?;
        }
        write!(f, "}}")
    }
}

impl Display for IrProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "global:")?;
        write!(f, "{}", self.glob)?;
        for func in self.funcs.values() {
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}
