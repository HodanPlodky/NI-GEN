use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum ValueCell {
    Register(usize),
    StackOffset(i64),
    Value(i64),
}

pub trait RegAllocator {
    fn get_location(&self, reg: middleend::inst::Register) -> ValueCell;
    fn get_used(&self) -> &Vec<usize>;
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
    pub fn new(prog: &middleend::ir::Function) -> Self {
        let mut res = Self {
            freeowned: vec![5, 6, 7, 28],
            registers: HashMap::new(),
            stacksize: 0,
        };
        res.allocate(prog);
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

    fn get_used(&self) -> &Vec<usize> {
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
    freeowned: Vec<usize>,
    registers: HashMap<middleend::inst::Register, ValueCell>,
    stacksize: i64,
}

impl LinearAllocator {
    pub fn new(prog: &middleend::ir::Function) -> Self {
        let mut res = Self {
            freeowned: vec![5, 6, 7, 28],
            registers: HashMap::new(),
            stacksize: 0,
        };
        res.allocate(prog);
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

impl RegAllocator for LinearAllocator {
    fn get_location(&self, reg: middleend::inst::Register) -> ValueCell {
        self.registers[&reg]
    }

    fn get_used(&self) -> &Vec<usize> {
        todo!()
    }

    fn get_stacksize(&self) -> usize {
        self.stacksize as usize
    }
}

/*
 * This one is a big boy
 */
pub struct ColoringAllocator;
