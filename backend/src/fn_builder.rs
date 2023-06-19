use std::collections::HashMap;

use crate::{
    insts::{AsmInstruction, Offset},
    AsmBasicBlock, AsmFunction, register_alloc::ValueCell,
};

pub type OffsetEnv = HashMap<middleend::inst::Register, Offset>;

pub struct AsmFunctionBuilder {
    pub name: String,
    stacksize: usize,
    pub actual_bb: usize,
    blocks: Vec<AsmBasicBlock>,

    registers: HashMap<middleend::inst::Register, ValueCell>,
    freeowned: Vec<usize>,
    freetemp: Vec<usize>,
    offsets: OffsetEnv,
}

impl AsmFunctionBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            stacksize: 0,
            actual_bb: 0,
            blocks: vec![],
            freeowned: vec![5, 6, 7, 28],
            freetemp: vec![29, 30, 31],

            registers: HashMap::new(),
            offsets: HashMap::new(),
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

    pub fn build(self) -> AsmFunction {
        if self.stacksize == 0 {
            return AsmFunction {
                name: self.name,
                blocks: self.blocks,
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

    pub fn allocate_reg(&mut self, reg: middleend::inst::Register) -> ValueCell {
        if self.freeowned.len() <= 0 {
            let offset = ValueCell::StackOffset(self.stacksize as i64);
            self.stacksize += 8;
            self.registers.insert(reg, offset);
            offset
        } else {
            let register = ValueCell::Register(self.freeowned.pop().unwrap());
            self.registers.insert(reg, register);
            register
        }
    }

    pub fn get_reg(&mut self, reg: middleend::inst::Register) -> usize {
        match self.registers.get(&reg) {
            Some(ValueCell::Register(reg)) => *reg,
            Some(ValueCell::StackOffset(offset)) => {
                let target = self.freetemp.pop().unwrap().clone();
                self.add_instruction(AsmInstruction::Ld(target, 2, *offset));
                target
            }
            None => unreachable!(),
        }
    }

    pub fn store_reg(&mut self, reg: middleend::inst::Register, tmpreg: usize) {
        match self.registers.get(&reg) {
            Some(ValueCell::Register(_)) => (),
            Some(ValueCell::StackOffset(offset)) => {
                self.add_instruction(AsmInstruction::Sd(tmpreg, 2, *offset))
            }
            None => unreachable!(),
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

    pub fn store_offset(&mut self, reg: middleend::inst::Register, offset: Offset) {
        self.offsets.insert(reg, offset);
    }

    pub fn get_offset(&mut self, reg: middleend::inst::Register) -> Option<Offset> {
        self.offsets.get(&reg).copied()
    }

    pub fn release_temp(&mut self) {
        self.freetemp = vec![29, 30, 31];
    }

    pub fn add_instruction(&mut self, inst: AsmInstruction) {
        self.blocks.last_mut().unwrap().push(inst);
    }
}
