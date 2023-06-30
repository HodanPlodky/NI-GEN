use middleend::inst::{
    ImmI, Instruction, Reg, RegReg, SymRegs, TerminatorBranch, TerminatorJump, TerminatorReg,
};

use crate::{insts::AsmInstruction, AsmFunctionBuilder};

pub fn basic_instruction_selection(inst: &Instruction, builder: &mut AsmFunctionBuilder) {
    let a: [usize; 8] = [10, 11, 12, 13, 14, 15, 16, 17];
    let reg = inst.id;
    match &inst.data {
        middleend::inst::InstructionType::Ldi(ImmI(imm)) => {
            let out = builder.get_reg(reg);
            builder.add_instruction(AsmInstruction::Addi(out, 0, *imm));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Ldc(_) => todo!(),
        middleend::inst::InstructionType::Ld(Reg(rs1)) => {
            let out = builder.get_reg(reg);
            let addr = builder.load_reg(*rs1);
            builder.add_instruction(AsmInstruction::Ld(out, addr, 0));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::St(RegReg(rs1, rs2)) => {
            let input = builder.load_reg(*rs2);
            let addr = builder.load_reg(*rs1);
            builder.add_instruction(AsmInstruction::Sd(input, addr, 0));
            builder.release_temp();
        }
        middleend::inst::InstructionType::Alloca(ImmI(_)) => (),
        middleend::inst::InstructionType::Allocg(_) => todo!(),
        middleend::inst::InstructionType::Cpy(_) => todo!(),
        middleend::inst::InstructionType::Gep(_) => todo!(),
        middleend::inst::InstructionType::Add(RegReg(r1, r2)) => {
            let out = builder.get_reg(reg);
            let asm_r1 = builder.load_reg(*r1);
            let asm_r2 = builder.load_reg(*r2);
            builder.add_instruction(AsmInstruction::Add(out, asm_r1, asm_r2));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Sub(RegReg(rs1, rs2)) => {
            let out = builder.get_reg(reg);
            let asm_r1 = builder.load_reg(*rs1);
            let asm_r2 = builder.load_reg(*rs2);
            builder.add_instruction(AsmInstruction::Sub(out, asm_r1, asm_r2));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Mul(RegReg(rs1, rs2)) => {
            let out = builder.get_reg(reg);
            let asm_r1 = builder.load_reg(*rs1);
            let asm_r2 = builder.load_reg(*rs2);
            builder.add_instruction(AsmInstruction::Mul(out, asm_r1, asm_r2));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Div(_) => todo!(),
        middleend::inst::InstructionType::Mod(_) => todo!(),
        middleend::inst::InstructionType::Shr(_) => todo!(),
        middleend::inst::InstructionType::Shl(_) => todo!(),
        middleend::inst::InstructionType::And(_) => todo!(),
        middleend::inst::InstructionType::Or(_) => todo!(),
        middleend::inst::InstructionType::Xor(_) => todo!(),
        middleend::inst::InstructionType::Neg(_) => todo!(),
        middleend::inst::InstructionType::Le(RegReg(rs1, rs2)) => {
            let out = builder.get_reg(reg);
            let asm1 = builder.load_reg(*rs1);
            let asm2 = builder.load_reg(*rs2);
            builder.add_instruction(AsmInstruction::Addi(out, asm2, 1));
            builder.add_instruction(AsmInstruction::Slt(out, asm1, out));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Lt(RegReg(rs1, rs2)) => {
            let out = builder.get_reg(reg);
            let asm1 = builder.load_reg(*rs1);
            let asm2 = builder.load_reg(*rs2);
            builder.add_instruction(AsmInstruction::Slt(out, asm1, asm2));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Gt(_) => todo!(),
        middleend::inst::InstructionType::Ge(_) => todo!(),
        middleend::inst::InstructionType::Eql(_) => todo!(),
        middleend::inst::InstructionType::Call(_) => todo!(),
        middleend::inst::InstructionType::CallDirect(SymRegs(sym, regs)) => {
            if regs.len() >= 8 {
                todo!();
            }
            for i in 0..regs.len() {
                let src = builder.load_reg(regs[i]);
                builder.add_instruction(AsmInstruction::Addi(a[i], src, 0));
                builder.release_temp();
            }
            let offset = builder.force_store(1);
            builder.add_instruction(AsmInstruction::Call(sym.clone()));
            builder.add_instruction(AsmInstruction::Ld(1, 2, offset));
            let out = builder.get_reg(reg);
            builder.add_instruction(AsmInstruction::Addi(out, a[0], 0));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Arg(ImmI(imm)) => {
            let out = builder.get_reg(reg);
            builder.add_instruction(AsmInstruction::Addi(out, a[*imm as usize], 0));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Ret(_) => builder.add_instruction(AsmInstruction::Ret),
        middleend::inst::InstructionType::Exit(_) => (),
        middleend::inst::InstructionType::Retr(TerminatorReg(reg)) => {
            let out = builder.get_reg(*reg);
            builder.add_instruction(AsmInstruction::Addi(a[0], out, 0));
            builder.add_instruction(AsmInstruction::Ret);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Jmp(TerminatorJump(bb_index)) => builder.add_instruction(
            AsmInstruction::Jal(0, *bb_index as i64, builder.name.clone()),
        ),
        middleend::inst::InstructionType::Branch(TerminatorBranch(reg, _, false_bb)) => {
            let input = builder.load_reg(*reg);
            builder.add_instruction(AsmInstruction::Beq(
                input,
                0,
                *false_bb as i64,
                builder.name.clone(),
            ));
            builder.release_temp();
        }
        middleend::inst::InstructionType::Print(_) => todo!(),
        middleend::inst::InstructionType::Phi(_) => todo!(),
    }
}
