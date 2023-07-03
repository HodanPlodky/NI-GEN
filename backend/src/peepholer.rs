use crate::{insts::AsmInstruction, AsmBasicBlock, AsmFunction};

pub trait Database {
    fn query(&self, insts: &[AsmInstruction]) -> Option<Vec<AsmInstruction>>;
}

pub struct MockDatabase;

impl Database for MockDatabase {
    fn query(&self, insts: &[AsmInstruction]) -> Option<Vec<AsmInstruction>> {
        match insts {
            &[AsmInstruction::Addi(reg, 0, imm), AsmInstruction::Add(out_reg, rs1, rs2)]
                if reg == rs2 =>
            {
                Some(vec![AsmInstruction::Addi(out_reg, rs1, imm)])
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

    fn find_and_replace(&self, block: &mut AsmBasicBlock, index: usize, size: usize) {
        loop {
            let result = self.database.query(&block[index..(index + size)]);
            match result {
                Some(rewrite) => {
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
    }

    pub fn pass_basicblock(&self, block: &mut AsmBasicBlock, size: usize) {
        let mut index = 0;
        while index + size <= block.len() {
            self.find_and_replace(block, index, size);
            index += 1;
        }
    }

    pub fn pass_function(&self, function: &mut AsmFunction, size: usize) {
        for block in &mut function.blocks {
            self.pass_basicblock(block, size);
        }
    }
}
