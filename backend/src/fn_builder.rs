use std::collections::{HashMap, HashSet};

use middleend::ir::Function;

use crate::{
    backend_ir::AsmBasicBlock,
    insts::{AsmInstruction, Offset, Rd},
    peepholer::PeepHoler,
    register_alloc::{RegAllocator, ValueCell, LinearAllocator},
    AsmFunction,
};

pub type OffsetEnv = HashMap<middleend::inst::Register, Offset>;

pub struct AsmFunctionBuilder<'a> {
    pub name: String,
    stacksize: usize,
    pub actual_bb: usize,
    blocks: Vec<AsmBasicBlock>,

    freetemp: Vec<usize>,
    ir_function : &'a Function,
}

impl<'a> AsmFunctionBuilder<'a> {
    pub fn new(name: String, ir_function : &'a Function) -> Self {
        Self {
            name,
            stacksize: 0,
            actual_bb: 0,
            blocks: vec![],

            freetemp: vec![29, 30, 31],
            ir_function,
        }
    }

    fn add_epilogue(block: AsmBasicBlock, stacksize: usize) -> AsmBasicBlock {
        let mut block = block;
        match block.last() {
            Some(AsmInstruction::Ret) => {
                block.pop();
                block.push(AsmInstruction::Addi(Rd::Sp, Rd::Sp, stacksize as i64));
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

    fn patch_ir_registers(reg_allocator : &dyn RegAllocator, block: AsmBasicBlock) -> AsmBasicBlock {
        let mut block = block;

        block
    }

    fn remove_unused_bb(block: &mut AsmBasicBlock, used: &HashSet<Rd>) -> bool {
        let mut change = false;
        let mut index = 0;
        while index < block.len() {
            match block[index].get_write() {
                Some(Rd::Ir(rd)) if !used.contains(&Rd::Ir(rd)) => {
                    block.remove(index);
                    change = true;
                }
                Some(_) | None => index += 1,
            }
        }
        change
    }

    fn get_used_regs(blocks: &Vec<AsmBasicBlock>) -> HashSet<Rd> {
        blocks
            .into_iter()
            .map(|x| x.iter().map(|x| x.get_reads()).flatten().collect::<Vec<Rd>>())
            .flatten()
            .collect()
    }

    fn remove_unused(blocks: &mut Vec<AsmBasicBlock>) -> bool {
        let mut change = false;
        loop {
            let used = AsmFunctionBuilder::get_used_regs(blocks);
            let mut inter_change = false;
            for block in blocks.iter_mut() {
                inter_change |= AsmFunctionBuilder::remove_unused_bb(block, &used);
            }
            change |= inter_change;
            if !inter_change {
                break;
            }
        }
        change
    }

    fn peepholer_run(peepholer: &PeepHoler, blocks : &mut Vec<AsmBasicBlock>) {
        loop {
            let mut change = false;
            for block in blocks.iter_mut() {
                change |= peepholer.pass_basicblock(block, 2);
            }
            change |= AsmFunctionBuilder::remove_unused(blocks);

            if !change {
                break;
            }
        }
    }

    pub fn build(self, peepholer: PeepHoler) -> AsmFunction {
        let mut blocks = self.blocks;
        //AsmFunctionBuilder::peepholer_run(&peepholer, &mut blocks);
        
        let used_regs = AsmFunctionBuilder::get_used_regs(&blocks);
        let reg_allocator = LinearAllocator::new(self.ir_function, used_regs);

        // do register allocation
        let blocks: Vec<AsmBasicBlock> = blocks
            .into_iter()
            .map(|x| AsmFunctionBuilder::patch_ir_registers(&reg_allocator, x))
            .collect();

        // epilogues
        let mut blocks: Vec<AsmBasicBlock> = blocks
            .into_iter()
            .map(|x| AsmFunctionBuilder::add_epilogue(x, self.stacksize))
            .collect();

        // prolog
        blocks.first_mut().expect("Totally empty function").insert(
            0,
            AsmInstruction::Addi(Rd::Sp, Rd::Sp, -(self.stacksize as i64)),
        );

        //AsmFunctionBuilder::peepholer_run(&peepholer, &mut blocks);

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
    //pub fn load_reg(&mut self, reg: middleend::inst::Register) -> usize {
        //match self.reg_allocator.get_location(reg) {
            //ValueCell::Register(reg) => reg,
            //ValueCell::StackOffset(offset) => {
                //let target = self.freetemp.pop().unwrap().clone();
                //self.add_instruction(AsmInstruction::Ld(Rd::Arch(target), Rd::Sp, offset));
                //target
            //}
            //ValueCell::Value(val) => {
                //let target = self.freetemp.pop().unwrap().clone();
                //self.add_instruction(AsmInstruction::Addi(Rd::Arch(target), Rd::Sp, val));
                //target
            //}
        //}
    //}
//
    //// get register, value is not guranteed
    //pub fn get_reg(&mut self, reg: middleend::inst::Register) -> usize {
        //match self.reg_allocator.get_location(reg) {
            //ValueCell::Register(reg) => reg,
            //ValueCell::StackOffset(_) => {
                //let target = self.freetemp.pop().unwrap().clone();
                //target
            //}
            //_ => unreachable!(),
        //}
    //}
//
    //pub fn store_reg(&mut self, reg: middleend::inst::Register, tmpreg: usize) {
        //match self.reg_allocator.get_location(reg) {
            //ValueCell::Register(_) => (),
            //ValueCell::StackOffset(offset) => {
                //self.add_instruction(AsmInstruction::Sd(Rd::Arch(tmpreg), Rd::Sp, offset))
            //}
            //_ => unreachable!(),
        //}
    //}

    pub fn force_store(&mut self, reg: Rd) -> Offset {
        let offset = self.stacksize;
        self.stacksize += 8;
        self.add_instruction(AsmInstruction::Sd(reg, Rd::Sp, offset as i64));
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
}
