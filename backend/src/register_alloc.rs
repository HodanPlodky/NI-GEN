use std::collections::{HashMap, HashSet};

use middleend::{
    analysis::{DataFlowAnalysis, LiveRegisterAnalysis},
    inst::BasicBlock,
};

#[derive(Clone, Copy)]
pub enum ValueCell {
    Register(usize),
    StackOffset(i64),
    Value(i64),
}

pub trait RegAllocator {
    fn get_location(&self, reg: middleend::inst::Register) -> ValueCell;
    fn get_used(&self, inst: middleend::inst::InstUUID) -> &Vec<usize>;
    fn get_stacksize(&self) -> usize;
}

/*
 * First gets the space and has it to the end of the
 * usage of this allocator
 */
pub struct NaiveAllocator {
    freeowned: Vec<usize>,
    registers: HashMap<middleend::inst::Register, ValueCell>,
    stacksize: i64,
}

impl NaiveAllocator {
    pub fn new(function: &middleend::ir::Function) -> Self {
        let mut res = Self {
            freeowned: vec![5, 6, 7, 28],
            registers: HashMap::new(),
            stacksize: 0,
        };
        res.allocate(function);
        res
    }

    fn allocate(&mut self, prog: &middleend::ir::Function) {
        for block in prog.blocks.iter() {
            for inst in block.iter() {
                match &inst.data {
                    middleend::inst::InstructionType::Alloca(middleend::inst::ImmI(size)) => {
                        self.registers
                            .insert(inst.id, ValueCell::Value(self.stacksize));
                        self.stacksize += size;
                    }
                    _ => self.allocate_reg(inst.id),
                }
            }
        }
    }

    fn allocate_reg(&mut self, reg: middleend::inst::Register) {
        if self.freeowned.len() <= 0 {
            let offset = ValueCell::StackOffset(self.stacksize);
            self.stacksize += 8;
            self.registers.insert(reg, offset);
        } else {
            let register = ValueCell::Register(self.freeowned.pop().unwrap());
            self.registers.insert(reg, register);
        }
    }
}

impl RegAllocator for NaiveAllocator {
    fn get_location(&self, reg: middleend::inst::Register) -> ValueCell {
        self.registers[&reg]
    }

    fn get_used(&self, inst: middleend::inst::InstUUID) -> &Vec<usize> {
        todo!()
    }

    fn get_stacksize(&self) -> usize {
        self.stacksize as usize
    }
}

/*
 * First gets the space but it only has
 * is for a duration of the lifetime of the ir register
 */
pub struct LinearAllocator {
    liveness: Vec<Vec<HashSet<middleend::inst::Register>>>,
    freeowned: Vec<usize>,
    used_register: Vec<usize>,
    registers: HashMap<middleend::inst::Register, ValueCell>,
    release: Vec<Vec<Vec<middleend::inst::Register>>>,
    used: Vec<Vec<Vec<usize>>>,
    stacksize: i64,
}

impl LinearAllocator {
    pub fn new(function: &middleend::ir::Function) -> Self {
        let mut liveanalysis = LiveRegisterAnalysis::new(function);
        let mut res = Self {
            liveness: liveanalysis.analyze(),
            freeowned: vec![5, 6, 7, 28],
            used_register: vec![],
            registers: HashMap::new(),
            release: function
                .blocks
                .iter()
                .map(|x| x.iter().map(|_| vec![]).collect())
                .collect(),
            used: function
                .blocks
                .iter()
                .map(|x| x.iter().map(|_| vec![]).collect())
                .collect(),
            stacksize: 0,
        };
        res.allocate(function);
        res
    }

    fn allocate(&mut self, prog: &middleend::ir::Function) {
        for block in prog.blocks.iter() {
            for inst in block.iter() {
                match &inst.data {
                    middleend::inst::InstructionType::Alloca(middleend::inst::ImmI(size)) => {
                        self.registers
                            .insert(inst.id, ValueCell::Value(self.stacksize));
                        self.stacksize += size;
                    }
                    _ => self.allocate_reg(inst.id, &prog.blocks),
                }
                self.release(inst.id);
            }
        }
    }

    fn allocate_reg(&mut self, reg: middleend::inst::Register, blocks: &Vec<BasicBlock>) {
        if self.freeowned.len() <= 0 {
            let offset = ValueCell::StackOffset(self.stacksize);
            self.stacksize += 8;
            self.registers.insert(reg, offset);
        } else {
            let reg_name = self.freeowned.pop().unwrap();
            self.used_register.push(reg_name);
            let register = ValueCell::Register(reg_name);
            self.registers.insert(reg, register);
            let (_, bb_index, inst_index) = reg;
            self.used[bb_index][inst_index] = self.used_register.clone();
            self.create_release(reg, blocks);
        }
    }

    fn create_release(&mut self, reg: middleend::inst::Register, blocks: &Vec<BasicBlock>) {
        let (_, bb_start, inst_start) = reg;
        let mut place = reg.clone();
        for bb_index in bb_start..blocks.len() {
            for inst_index in inst_start..blocks[bb_index].len() {
                if self.liveness[bb_index][inst_index].contains(&reg) {
                    place = (false, bb_index, inst_index);
                }
            }
        }
        let (_, bb_index, inst_index) = place;
        self.release[bb_index][inst_index].push(reg);
    }

    fn release(&mut self, reg: middleend::inst::Register) {
        let (_, bb_index, inst_index) = reg;
        for rel_reg in self.release[bb_index][inst_index].iter() {
            match self.get_location(*rel_reg) {
                ValueCell::Register(reg) => {
                    for i in 0..self.used_register.len() {
                        if self.used_register[i] == reg {
                            self.used_register.remove(i);
                            break;
                        }
                    }
                    self.freeowned.push(reg)
                }
                _ => (),
            }
        }
    }
}

impl RegAllocator for LinearAllocator {
    fn get_location(&self, reg: middleend::inst::Register) -> ValueCell {
        self.registers[&reg]
    }

    fn get_used(&self, inst: middleend::inst::InstUUID) -> &Vec<usize> {
        let (_, bb_index, inst_index) = inst;
        &self.used[bb_index][inst_index]
    }

    fn get_stacksize(&self) -> usize {
        self.stacksize as usize
    }
}

/*
 * This one is a big boy
 */
pub struct ColoringAllocator;
