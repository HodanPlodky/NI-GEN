use std::collections::HashMap;

use crate::{
    insts::{AsmInstruction, Offset},
    peepholer::PeepHoler,
    register_alloc::{RegAllocator, ValueCell},
    AsmBasicBlock, AsmFunction,
};

pub type OffsetEnv = HashMap<middleend::inst::Register, Offset>;

pub struct AsmFunctionBuilder<'a> {
    pub name: String,
    stacksize: usize,
    pub actual_bb: usize,
    blocks: Vec<AsmBasicBlock>,

    reg_allocator: &'a dyn RegAllocator,
    freetemp: Vec<usize>,
}

impl<'a> AsmFunctionBuilder<'a> {
    pub fn new(name: String, reg_allocator: &'a dyn RegAllocator) -> Self {
        Self {
            name,
            stacksize: reg_allocator.get_stacksize(),
            actual_bb: 0,
            blocks: vec![],

            reg_allocator,
            freetemp: vec![29, 30, 31],
        }
    }

    fn add_epilogue(block: AsmBasicBlock, stacksize: usize) -> AsmBasicBlock {
        let mut block = block;
        match block.last() {
            Some(AsmInstruction::Ret) => {
                block.pop();
                block.push(AsmInstruction::Addi(2, 2, stacksize as i64));
                block.push(AsmInstruction::Ret);
                block
            }
            _ => block,
        }
    }

    /*
     * This is here because of the compress instructions set
     * in the RISCV spec, I just elected to ignore them
     */
    fn bb_size(block: &AsmBasicBlock) -> usize {
        block.len() * 4
        //block.iter().map(|x| x.size()).sum()
    }

    fn patch_jumps(offsets: &Vec<usize>, block: AsmBasicBlock) -> AsmBasicBlock {
        let mut block = block;
        match block.last_mut() {
            Some(AsmInstruction::Jal(_, offset, _))
            | Some(AsmInstruction::Jalr(_, _, offset))
            | Some(AsmInstruction::Beq(_, _, offset, _))
            | Some(AsmInstruction::Bne(_, _, offset, _)) => {
                *offset = offsets[*offset as usize] as i64;
            }
            _ => (),
        };
        block
    }

    pub fn build(self, peepholer: PeepHoler) -> AsmFunction {
        if self.stacksize == 0 {
            let mut blocks = self.blocks;
            for _ in 0..10 {
                for block in &mut blocks {
                    peepholer.pass_basicblock(block, 2);
                }
            }
            return AsmFunction {
                name: self.name,
                blocks,
            };
        }

        // epilogues
        let mut blocks: Vec<AsmBasicBlock> = self
            .blocks
            .into_iter()
            .map(|x| AsmFunctionBuilder::add_epilogue(x, self.stacksize))
            .collect();

        // prolog
        blocks
            .first_mut()
            .expect("Totally empty function")
            .insert(0, AsmInstruction::Addi(2, 2, -(self.stacksize as i64)));

        for _ in 0..10 {
            for block in &mut blocks {
                peepholer.pass_basicblock(block, 2);
            }
        }

        let lens: Vec<usize> = blocks
            .iter()
            .map(|x| AsmFunctionBuilder::bb_size(x))
            .collect();

        let mut offsets: Vec<usize> = vec![];
        let mut act = 0;
        for len in lens {
            offsets.push(act);
            act += len;
        }

        let blocks: Vec<AsmBasicBlock> = blocks
            .into_iter()
            .map(|x| AsmFunctionBuilder::patch_jumps(&offsets, x))
            .collect();

        AsmFunction {
            name: self.name,
            blocks,
        }
    }

    pub fn create_block(&mut self) -> usize {
        self.blocks.push(AsmBasicBlock::new());
        self.blocks.len() - 1
    }

    // get register with values stored in ValueCell
    pub fn load_reg(&mut self, reg: middleend::inst::Register) -> usize {
        match self.reg_allocator.get_location(reg) {
            ValueCell::Register(reg) => reg,
            ValueCell::StackOffset(offset) => {
                let target = self.freetemp.pop().unwrap().clone();
                self.add_instruction(AsmInstruction::Ld(target, 2, offset));
                target
            }
            ValueCell::Value(val) => {
                let target = self.freetemp.pop().unwrap().clone();
                self.add_instruction(AsmInstruction::Addi(target, 2, val));
                target
            }
        }
    }

    // get register, value is not guranteed
    pub fn get_reg(&mut self, reg: middleend::inst::Register) -> usize {
        match self.reg_allocator.get_location(reg) {
            ValueCell::Register(reg) => reg,
            ValueCell::StackOffset(_) => {
                let target = self.freetemp.pop().unwrap().clone();
                target
            }
            _ => unreachable!(),
        }
    }

    pub fn store_reg(&mut self, reg: middleend::inst::Register, tmpreg: usize) {
        match self.reg_allocator.get_location(reg) {
            ValueCell::Register(_) => (),
            ValueCell::StackOffset(offset) => {
                self.add_instruction(AsmInstruction::Sd(tmpreg, 2, offset))
            }
            _ => unreachable!(),
        }
    }

    pub fn force_store(&mut self, reg: usize) -> Offset {
        let offset = self.stacksize;
        self.stacksize += 8;
        self.add_instruction(AsmInstruction::Sd(reg, 2, offset as i64));
        offset as i64
    }

    pub fn allocate_stack(&mut self, size: i64) -> Offset {
        let offset = self.stacksize;
        self.stacksize += size as usize;
        offset as i64
    }

    pub fn release_temp(&mut self) {
        self.freetemp = vec![29, 30, 31];
    }

    pub fn add_instruction(&mut self, inst: AsmInstruction) {
        self.blocks.last_mut().unwrap().push(inst);
    }

    pub fn store_used(&mut self, inst: middleend::inst::InstUUID) -> Vec<Offset> {
        let used = self.reg_allocator.get_used(inst);
        let mut result = vec![];
        for reg in used {
            result.push(self.force_store(*reg));
        }
        result
    }

    pub fn load_used(&mut self, inst: middleend::inst::InstUUID, offsets: Vec<Offset>) {
        let used = self.reg_allocator.get_used(inst);
        for (reg, offset) in used.iter().zip(offsets.iter()) {
            self.add_instruction(AsmInstruction::Ld(*reg, 2, *offset));
        }
    }
}
