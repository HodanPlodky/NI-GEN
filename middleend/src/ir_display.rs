use std::fmt::Display;

use crate::{
    inst::{
        ImmC, ImmI, ImmIRegs, InstructionType, Reg, RegReg, RegRegImm, RegRegs, SymRegs,
        TerminatorBranch, TerminatorJump, TerminatorReg,
    },
    ir::{BasicBlock, Function, InstStore, Instruction, IrProgram, RegType, Register},
};

fn reg_view(reg: Register) -> String {
    let id = reg.val();
    format!("%{id}")
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionType::Ldi(ImmI(n)) => write!(f, "ldi {}", n),
            InstructionType::Ldc(ImmC(n)) => write!(f, "ldc {}", n),
            InstructionType::Ld(Reg(reg)) => write!(f, "ld [{}]", reg_view(*reg)),
            InstructionType::St(RegReg(addr, val)) => {
                write!(f, "store [{}] {}", reg_view(*addr), reg_view(*val))
            }
            InstructionType::Alloca(ImmI(n)) => write!(f, "alloca {}", n),
            InstructionType::Allocg(_) => todo!(),
            InstructionType::Mov(Reg(reg)) => write!(f, "mov {}", reg_view(*reg)),
            InstructionType::Gep(size, RegRegImm(start, index, imm)) => write!(
                f,
                "gep <{}> [{}] {} {}",
                size,
                reg_view(*start),
                reg_view(*index),
                imm
            ),
            InstructionType::Add(RegReg(l, r)) => {
                write!(f, "add {} {}", reg_view(*l), reg_view(*r))
            }
            InstructionType::Sub(RegReg(l, r)) => {
                write!(f, "sub {} {}", reg_view(*l), reg_view(*r))
            }
            InstructionType::Mul(RegReg(l, r)) => {
                write!(f, "mul {} {}", reg_view(*l), reg_view(*r))
            }
            InstructionType::Div(RegReg(l, r)) => {
                write!(f, "div {} {}", reg_view(*l), reg_view(*r))
            }
            InstructionType::Mod(RegReg(l, r)) => {
                write!(f, "mod {} {}", reg_view(*l), reg_view(*r))
            }
            InstructionType::Shr(RegReg(l, r)) => {
                write!(f, "shr {} {}", reg_view(*l), reg_view(*r))
            }
            InstructionType::Shl(RegReg(l, r)) => {
                write!(f, "shl {} {}", reg_view(*l), reg_view(*r))
            }
            InstructionType::And(RegReg(l, r)) => {
                write!(f, "and {} {}", reg_view(*l), reg_view(*r))
            }
            InstructionType::Or(RegReg(l, r)) => write!(f, "or {} {}", reg_view(*l), reg_view(*r)),
            InstructionType::Xor(RegReg(l, r)) => {
                write!(f, "xor {} {}", reg_view(*l), reg_view(*r))
            }
            InstructionType::Neg(Reg(reg)) => write!(f, "neg {}", reg_view(*reg)),
            InstructionType::Lt(RegReg(l, r)) => write!(f, "lt {} {}", reg_view(*l), reg_view(*r)),
            InstructionType::Le(RegReg(l, r)) => write!(f, "le {} {}", reg_view(*l), reg_view(*r)),
            InstructionType::Gt(RegReg(l, r)) => write!(f, "gt {} {}", reg_view(*l), reg_view(*r)),
            InstructionType::Ge(RegReg(l, r)) => write!(f, "ge {} {}", reg_view(*l), reg_view(*r)),
            InstructionType::Eql(RegReg(l, r)) => {
                write!(f, "eql {} {}", reg_view(*l), reg_view(*r))
            }
            InstructionType::Call(RegRegs(reg, regs)) => write!(
                f,
                "call {} [{}]",
                reg_view(*reg),
                regs.into_iter()
                    .map(|x| reg_view(*x))
                    .fold("".to_string(), |acc, x| acc + x.as_str())
            ),
            InstructionType::CallDirect(SymRegs(sym, regs)) => write!(
                f,
                "calldirect {} [{}]",
                sym,
                regs.into_iter()
                    .map(|x| reg_view(*x))
                    .fold("".to_string(), |acc, x| acc + x.as_str())
            ),
            InstructionType::Arg(ImmI(index)) => write!(f, "arg {}", index),
            InstructionType::Ret(_) => write!(f, "ret"),
            InstructionType::Retr(TerminatorReg(reg)) => write!(f, "retr {}", reg_view(*reg)),
            InstructionType::Jmp(TerminatorJump(to)) => write!(f, "jmp BB{}", to),
            InstructionType::Branch(TerminatorBranch(reg, true_bb, false_bb)) => {
                write!(f, "branch {} BB{} BB{}", reg_view(*reg), true_bb, false_bb)
            }
            InstructionType::Print(_) => todo!(),
            InstructionType::Phi(_) => todo!(),
            InstructionType::Exit(_) => write!(f, "exit"),
            InstructionType::SysCall(ImmIRegs(num, regs)) => write!(
                f,
                "syscall {} [{}]",
                num,
                regs.into_iter()
                    .map(|x| reg_view(*x))
                    .fold("".to_string(), |acc, x| acc + x.as_str())
            ),
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
            let reg_string = reg_view(self.id);
            write!(f, "{} : {} = {}", reg_string, self.reg_type, self.data)
        }
    }
}

/*
impl Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.instruction.iter() {
            writeln!(f, "\t{}", i)?;
        }
        Ok(())
    }
}
*/
impl BasicBlock {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, store: &InstStore) -> std::fmt::Result {
        for i in self.instruction.iter() {
            writeln!(f, "\t{}", store.get(*i))?;
        }
        Ok(())
    }
}

impl Function {
    pub fn display(&self, f: &mut std::fmt::Formatter<'_>, store: &InstStore) -> std::fmt::Result {
        writeln!(
            f,
            "function {}({}) : {} {{",
            self.name, self.arg_count, self.ret_type
        )?;
        for i in 0..self.blocks.len() {
            writeln!(f, "BB{}:", i)?;
            self.blocks[i].display(f, store)?;
        }
        write!(f, "}}")
    }
}

impl Display for IrProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "global:")?;
        self.glob.display(f, &self.store)?;
        for func in self.funcs.values() {
            writeln!(f, "")?;
            func.display(f, &self.store)?;
        }
        Ok(())
    }
}
