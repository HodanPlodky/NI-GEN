use crate::{backend_ir::AsmBasicBlock, insts::AsmInstruction, AsmFunction};

pub trait Database {
    fn query(&self, insts: &[AsmInstruction]) -> Option<Vec<AsmInstruction>>;
}

pub struct MockDatabase;

impl Database for MockDatabase {
    fn query(&self, insts: &[AsmInstruction]) -> Option<Vec<AsmInstruction>> {
        use crate::insts::Rd::*;
        match insts {
            &[AsmInstruction::Addi(rd1, rs1, 0)] if rd1 == rs1 => Some(vec![]),
            &[AsmInstruction::Addi(reg, Zero, imm), AsmInstruction::Add(out_reg, rs1, rs2)]
                if reg == rs2 =>
            {
                Some(vec![
                    AsmInstruction::Addi(reg, Zero, imm),
                    AsmInstruction::Addi(out_reg, rs1, imm),
                ])
            }
            &[AsmInstruction::Addi(reg, Zero, imm), AsmInstruction::Sub(out_reg, rs1, rs2)]
                if reg == rs2 =>
            {
                Some(vec![
                    AsmInstruction::Addi(reg, Zero, imm),
                    AsmInstruction::Addi(out_reg, rs1, -imm),
                ])
            }
            &[AsmInstruction::Addi(reg, rs, imm1), AsmInstruction::Addi(out_reg, rs1, imm2)]
                if reg == rs1 && reg != rs =>
            {
                Some(vec![AsmInstruction::Addi(out_reg, rs, imm1 + imm2)])
            }
            &[AsmInstruction::Addi(reg, Sp, imm), AsmInstruction::Ld(out_reg, rs1, 0)]
                if reg == rs1 && reg != Sp =>
            {
                Some(vec![
                    AsmInstruction::Addi(reg, Sp, imm),
                    AsmInstruction::Ld(out_reg, Sp, imm),
                ])
            }
            &[AsmInstruction::Addi(reg, Sp, imm), AsmInstruction::Sd(out_reg, rs1, 0)]
                if reg == rs1 && reg != Sp =>
            {
                Some(vec![
                    AsmInstruction::Addi(reg, Sp, imm),
                    AsmInstruction::Sd(out_reg, Sp, imm),
                ])
            }
            &[AsmInstruction::Sd(rd_sd, rs_sd, offset_sd), AsmInstruction::Ld(rd_ld, rs_ld, offset_ld)]
                if rs_sd == rs_ld && offset_sd == offset_ld =>
            {
                Some(vec![
                    AsmInstruction::Sd(rd_sd, rs_sd, offset_sd),
                    AsmInstruction::Addi(rd_ld, rd_sd, 0),
                ])
            }
            _ => None,
        }
    }
}

pub struct PeepHoler<'a> {
    database: &'a dyn Database,
}

impl<'a> PeepHoler<'a> {
    pub fn new(database: &'a dyn Database) -> Self {
        Self { database }
    }

    fn find_and_replace(&self, block: &mut AsmBasicBlock, index: usize, size: usize) -> bool {
        let mut change = false;
        while index + size < block.len() {
            let result = self.database.query(&block[index..(index + size)]);
            match result {
                Some(rewrite) => {
                    change = true;
                    for _ in 0..size {
                        block.remove(index);
                    }
                    for i in 0..rewrite.len() {
                        block.insert(index + i, rewrite[i].clone());
                    }
                }
                None => break,
            }
        }
        change
    }

    pub fn pass_basicblock(&self, block: &mut AsmBasicBlock, size: usize) -> bool {
        let mut change = false;
        for s in 1..=size {
            let mut index = 0;
            while index + size <= block.len() {
                change |= self.find_and_replace(block, index, s);
                index += 1;
            }
        }
        change
    }

    pub fn pass_function(&self, function: &mut AsmFunction, size: usize) -> bool {
        let mut change = false;
        for block in &mut function.blocks {
            change |= self.pass_basicblock(block, size);
        }
        change
    }
}
