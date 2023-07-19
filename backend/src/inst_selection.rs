use middleend::{
    inst::{ImmI, Reg, RegReg, SymRegs, TerminatorBranch, TerminatorJump, TerminatorReg},
    ir::Instruction,
};

use crate::{insts::AsmInstruction, AsmFunctionBuilder};

pub fn basic_instruction_selection(inst: &Instruction, builder: &mut AsmFunctionBuilder) {
    use crate::insts::Rd::*;
    let reg = inst.id;
    match &inst.data {
        &middleend::inst::InstructionType::Ldi(ImmI(imm)) => {
            builder.add_instruction(AsmInstruction::Addi(Ir(reg), Zero, imm));
        }
        &middleend::inst::InstructionType::Ldc(_) => todo!(),
        &middleend::inst::InstructionType::Ld(Reg(rs1)) => {
            builder.add_instruction(AsmInstruction::Ld(Ir(reg), Ir(rs1), 0));
        }
        &middleend::inst::InstructionType::St(RegReg(rs1, rs2)) => {
            builder.add_instruction(AsmInstruction::Sd(Ir(rs2), Ir(rs1), 0));
            builder.release_temp();
        }
        &middleend::inst::InstructionType::Alloca(ImmI(_)) => (),
        &middleend::inst::InstructionType::Allocg(_) => todo!(),
        &middleend::inst::InstructionType::Cpy(_) => todo!(),
        &middleend::inst::InstructionType::Gep(_) => todo!(),
        &middleend::inst::InstructionType::Add(RegReg(rs1, rs2)) => {
            builder.add_instruction(AsmInstruction::Add(Ir(reg), Ir(rs1), Ir(rs2)));
        }
        &middleend::inst::InstructionType::Sub(RegReg(rs1, rs2)) => {
            builder.add_instruction(AsmInstruction::Sub(Ir(reg), Ir(rs1), Ir(rs2)));
        }
        &middleend::inst::InstructionType::Mul(RegReg(rs1, rs2)) => {
            builder.add_instruction(AsmInstruction::Mul(Ir(reg), Ir(rs1), Ir(rs2)));
        }
        middleend::inst::InstructionType::Div(_) => todo!(),
        middleend::inst::InstructionType::Mod(_) => todo!(),
        middleend::inst::InstructionType::Shr(_) => todo!(),
        middleend::inst::InstructionType::Shl(_) => todo!(),
        middleend::inst::InstructionType::And(_) => todo!(),
        middleend::inst::InstructionType::Or(_) => todo!(),
        middleend::inst::InstructionType::Xor(_) => todo!(),
        middleend::inst::InstructionType::Neg(_) => todo!(),
        &middleend::inst::InstructionType::Le(RegReg(rs1, rs2)) => {
            builder.add_instruction(AsmInstruction::Addi(Arch(31), Ir(rs2), 1));
            builder.add_instruction(AsmInstruction::Slt(Ir(reg), Ir(rs1), Arch(31)));
        }
        &middleend::inst::InstructionType::Lt(RegReg(rs1, rs2)) => {
            builder.add_instruction(AsmInstruction::Slt(Ir(reg), Ir(rs1), Ir(rs2)));
        }
        &middleend::inst::InstructionType::Gt(_) => todo!(),
        &middleend::inst::InstructionType::Ge(_) => todo!(),
        &middleend::inst::InstructionType::Eql(_) => todo!(),
        &middleend::inst::InstructionType::Call(_) => todo!(),
        middleend::inst::InstructionType::CallDirect(SymRegs(sym, regs)) => {
            if regs.len() >= 8 {
                todo!();
            }
            // TODO implement working solution for stack passed arguments
            for i in 0..regs.len() {
                builder.add_instruction(AsmInstruction::Addi(ArgReg(i as u8), Ir(regs[i]), 0));
            }
            let offset = builder.force_store(Ra);
            builder.add_instruction(AsmInstruction::Call(sym.clone(), inst.id));
            builder.add_instruction(AsmInstruction::Ld(Ra, Sp, offset));
            builder.add_instruction(AsmInstruction::Addi(Ir(reg), ArgReg(0), 0));
        }
        &middleend::inst::InstructionType::Arg(ImmI(imm)) => {
            // TODO implement working solution for stack passed arguments
            builder.add_instruction(AsmInstruction::Addi(Ir(reg), ArgReg(imm as u8), 0));
        }
        &middleend::inst::InstructionType::Ret(_) => builder.add_instruction(AsmInstruction::Ret),
        &middleend::inst::InstructionType::Exit(_) => (),
        &middleend::inst::InstructionType::Retr(TerminatorReg(reg)) => {
            builder.add_instruction(AsmInstruction::Addi(ArgReg(0), Ir(reg), 0));
            builder.add_instruction(AsmInstruction::Ret);
        }
        &middleend::inst::InstructionType::Jmp(TerminatorJump(bb_index)) => builder
            .add_instruction(AsmInstruction::Jal(
                Zero,
                bb_index as i64,
                builder.name.clone(),
            )),
        &middleend::inst::InstructionType::Branch(TerminatorBranch(reg, _, false_bb)) => {
            builder.add_instruction(AsmInstruction::Beq(
                Ir(reg),
                Zero,
                false_bb as i64,
                builder.name.clone(),
            ));
            builder.release_temp();
        }
        middleend::inst::InstructionType::Print(_) => todo!(),
        middleend::inst::InstructionType::Phi(_) => todo!(),
    }
}
