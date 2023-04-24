use std::fmt::Display;

use crate::{
    inst::{BasicBlock, Instruction, InstructionType, RegType, Register, ImmI, Reg, TerminatorReg, RegReg, ImmC},
    ir::{Function, IrProgram},
};

fn reg_view(reg : Register) -> String {
    if reg.0 {
        "g(".to_string()
            + reg.1.to_string().as_str()
            + ","
            + reg.2.to_string().as_str()
            + ")"
    } else {
        "(".to_string()
            + reg.1.to_string().as_str()
            + ","
            + reg.2.to_string().as_str()
            + ")"
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionType::Ldi(ImmI(n)) => write!(f, "ldi {}", n),
            InstructionType::Ldc(ImmC(n)) => write!(f, "ldc {}", n),
            InstructionType::Ld(Reg(reg)) => write!(f, "ld [{}]", reg_view(*reg)), 
            InstructionType::St(RegReg(addr, val)) => write!(f, "store [{}] {}", reg_view(*addr), reg_view(*val)),
            InstructionType::Alloca(ImmI(n)) => write!(f, "alloca {}", n), 
            InstructionType::Allocg(_) => todo!(),
            InstructionType::Cpy(_) => todo!(),
            InstructionType::Gep(_) => todo!(),
            InstructionType::Add(RegReg(l, r)) => write!(f, "add {} {}", reg_view(*l), reg_view(*r)),
            InstructionType::Sub(RegReg(l, r)) => write!(f, "sub {} {}", reg_view(*l), reg_view(*r)),
            InstructionType::Mul(RegReg(l, r)) => write!(f, "mul {} {}", reg_view(*l), reg_view(*r)),
            InstructionType::Div(RegReg(l, r)) => write!(f, "div {} {}", reg_view(*l), reg_view(*r)),
            InstructionType::Mod(RegReg(l, r)) => write!(f, "mod {} {}", reg_view(*l), reg_view(*r)),
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
            InstructionType::Retr(TerminatorReg(reg)) => write!(f, "retr {}", reg_view(*reg)),
            InstructionType::Jmp(_) => todo!(),
            InstructionType::Branch(_) => todo!(),
            InstructionType::Print(_) => todo!(),
            InstructionType::Phi(_) => todo!(),
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
        if self.reg_type == RegType::Void {
            write!(f, "{}", self.data)
        } else {
            let reg_string =  reg_view(self.id);
            write!(f, "{} : {} = {}", reg_string, self.reg_type, self.data)
        }
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
            write!(f, "{}", self.blocks[i])?;
        }
        write!(f, "}}")
    }
}

impl Display for IrProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "global:")?;
        write!(f, "{}", self.glob)?;
        for func in self.funcs.values() {
            writeln!(f, "")?;
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}
