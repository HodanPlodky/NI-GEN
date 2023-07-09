use std::collections::{HashMap, HashSet};

use middleend::{
    analysis::{DataFlowAnalysis, LiveRegisterAnalysis},
    ir::Function,
};

use crate::{
    backend_ir::AsmBasicBlock,
    insts::{AsmInstruction, Offset, Rd},
    peepholer::PeepHoler,
    register_alloc::{LinearAllocator, RegAllocator, ValueCell},
    AsmFunction,
};

pub type OffsetEnv = HashMap<middleend::inst::Register, Offset>;

pub struct AsmFunctionBuilder<'a> {
    pub name: String,
    stacksize: usize,
    pub actual_bb: usize,
    blocks: Vec<AsmBasicBlock>,
    liveness: Vec<Vec<HashSet<middleend::inst::Register>>>,

    freetemp: Vec<usize>,
    ir_function: &'a Function,
}

impl<'a> AsmFunctionBuilder<'a> {
    pub fn new(name: String, ir_function: &'a Function) -> Self {
        let mut lifeanalysis = LiveRegisterAnalysis::new(ir_function);
        Self {
            liveness: lifeanalysis.analyze(),
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
            | Some(AsmInstruction::Blt(_, _, offset, _))
            | Some(AsmInstruction::Bge(_, _, offset, _))
            | Some(AsmInstruction::Bne(_, _, offset, _)) => {
                *offset = offsets[*offset as usize] as i64;
            }
            _ => (),
        };
        block
    }

    fn replace_ir_reg(
        reg_allocator: &dyn RegAllocator,
        reg: Rd,
        load: bool,
        tmp_regs: &mut Vec<usize>,
    ) -> (Vec<AsmInstruction>, Rd, Vec<AsmInstruction>) {
        match reg {
            Rd::Ir(ir_reg) => match reg_allocator.get_location(ir_reg) {
                ValueCell::Register(reg) => (vec![], Rd::Arch(reg), vec![]),
                ValueCell::StackOffset(offset) => {
                    let target = tmp_regs.pop().unwrap().clone();
                    let before = if load {
                        vec![AsmInstruction::Ld(Rd::Arch(target), Rd::Sp, offset)]
                    } else {
                        vec![]
                    };
                    let after = if load {
                        vec![]
                    } else {
                        vec![AsmInstruction::Sd(Rd::Arch(target), Rd::Sp, offset)]
                    };

                    (before, Rd::Arch(target), after)
                }
                ValueCell::Value(value) if load => {
                    let target = tmp_regs.pop().unwrap().clone();
                    (
                        vec![AsmInstruction::Addi(Rd::Arch(target), Rd::Sp, value)],
                        Rd::Arch(target),
                        vec![],
                    )
                }
                x => {
                    println!("{:?}", x);
                    unreachable!()
                }
            },
            _ => (vec![], reg, vec![]),
        }
    }

    fn patch_ir_register_inst(
        reg_allocator: &dyn RegAllocator,
        inst: AsmInstruction,
        block: &mut AsmBasicBlock,
        stacksize: Offset,
    ) -> Offset {
        let mut temp = vec![29, 30, 31];
        let mut inst = inst;

        match &mut inst {
            AsmInstruction::Lb(_, rs, offset)
            | AsmInstruction::Lh(_, rs, offset)
            | AsmInstruction::Lw(_, rs, offset)
            | AsmInstruction::Ld(_, rs, offset)
            | AsmInstruction::Lbu(_, rs, offset)
            | AsmInstruction::Lhu(_, rs, offset)
            | AsmInstruction::Sb(_, rs, offset)
            | AsmInstruction::Sh(_, rs, offset)
            | AsmInstruction::Sw(_, rs, offset)
            | AsmInstruction::Sd(_, rs, offset) => {
                if let Rd::Ir(ir_rs) = rs {
                    match reg_allocator.get_location(ir_rs.clone()) {
                        ValueCell::Value(value) => {
                            *rs = Rd::Sp;
                            *offset += value;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }

        let (mut before, load_regs, _) = inst
            .get_reads()
            .into_iter()
            .map(|x| AsmFunctionBuilder::replace_ir_reg(reg_allocator, x, true, &mut temp))
            .fold((vec![], vec![], vec![]), |acc, x| {
                let (mut b, mut i, mut a) = acc;
                let (mut bx, ix, mut ax) = x;
                b.append(&mut bx);
                i.push(ix);
                a.append(&mut ax);
                (b, i, a)
            });

        let (_, write_regs, mut after) = inst
            .get_write()
            .into_iter()
            .map(|x| AsmFunctionBuilder::replace_ir_reg(reg_allocator, x, false, &mut temp))
            .fold((vec![], vec![], vec![]), |acc, x| {
                let (mut b, mut i, mut a) = acc;
                let (mut bx, ix, mut ax) = x;
                b.append(&mut bx);
                i.push(ix);
                a.append(&mut ax);
                (b, i, a)
            });

        match &mut inst {
            AsmInstruction::Lui(_, _) => todo!(),
            AsmInstruction::Auipc(_, _) => todo!(),
            AsmInstruction::Jal(rd, _, _) => *rd = write_regs[0],
            AsmInstruction::Jalr(rd, rs, _) => {
                *rd = write_regs[0];
                *rs = load_regs[0];
            }
            AsmInstruction::Beq(rs1, rs2, _, _)
            | AsmInstruction::Bne(rs1, rs2, _, _)
            | AsmInstruction::Blt(rs1, rs2, _, _)
            | AsmInstruction::Bge(rs1, rs2, _, _)
            | AsmInstruction::Bltu(rs1, rs2, _, _)
            | AsmInstruction::Bgeu(rs1, rs2, _, _) => {
                *rs1 = load_regs[0];
                *rs2 = load_regs[1];
            }

            AsmInstruction::Addi(rd, rs, _)
            | AsmInstruction::Slti(rd, rs, _)
            | AsmInstruction::Sltiu(rd, rs, _)
            | AsmInstruction::Xori(rd, rs, _)
            | AsmInstruction::Ori(rd, rs, _)
            | AsmInstruction::Andi(rd, rs, _)
            | AsmInstruction::Slli(rd, rs, _)
            | AsmInstruction::Srli(rd, rs, _)
            | AsmInstruction::Srai(rd, rs, _) => {
                *rd = write_regs[0];
                *rs = load_regs[0];
            }
            AsmInstruction::Add(rd, rs1, rs2)
            | AsmInstruction::Mul(rd, rs1, rs2)
            | AsmInstruction::Sub(rd, rs1, rs2)
            | AsmInstruction::Sll(rd, rs1, rs2)
            | AsmInstruction::Srl(rd, rs1, rs2)
            | AsmInstruction::Slt(rd, rs1, rs2)
            | AsmInstruction::Sltu(rd, rs1, rs2)
            | AsmInstruction::Xor(rd, rs1, rs2)
            | AsmInstruction::Or(rd, rs1, rs2)
            | AsmInstruction::And(rd, rs1, rs2)
            | AsmInstruction::Sra(rd, rs1, rs2) => {
                *rd = write_regs[0];
                *rs1 = load_regs[0];
                *rs2 = load_regs[1];
            }

            AsmInstruction::Lb(rd, rs, _)
            | AsmInstruction::Lh(rd, rs, _)
            | AsmInstruction::Lw(rd, rs, _)
            | AsmInstruction::Ld(rd, rs, _)
            | AsmInstruction::Lbu(rd, rs, _)
            | AsmInstruction::Lhu(rd, rs, _) => {
                *rd = write_regs[0];
                *rs = load_regs[0];
            }
            AsmInstruction::Sb(rs1, rs2, _)
            | AsmInstruction::Sh(rs1, rs2, _)
            | AsmInstruction::Sw(rs1, rs2, _)
            | AsmInstruction::Sd(rs1, rs2, _) => {
                *rs1 = load_regs[0];
                *rs2 = load_regs[1];
            }
            _ => (),
        };

        let mut stack_added = 0;
        if let AsmInstruction::Call(_, inst_id) = inst {
            let used = reg_allocator.get_used(inst_id);
            for reg in used {
                before.push(AsmInstruction::Sd(
                    Rd::Arch(*reg),
                    Rd::Sp,
                    stacksize + stack_added,
                ));
                after.insert(
                    0,
                    AsmInstruction::Ld(Rd::Arch(*reg), Rd::Sp, stacksize + stack_added),
                );
                stack_added += 8;
            }
        }

        block.append(&mut before);
        block.push(inst);
        block.append(&mut after);
        stack_added
    }

    fn patch_ir_registers(
        reg_allocator: &dyn RegAllocator,
        block: AsmBasicBlock,
        stack_size: Offset,
    ) -> (AsmBasicBlock, Offset) {
        let mut result = vec![];
        let mut biggest_stack = 0;
        for inst in block {
            biggest_stack = std::cmp::max(
                AsmFunctionBuilder::patch_ir_register_inst(
                    reg_allocator,
                    inst,
                    &mut result,
                    stack_size,
                ),
                biggest_stack,
            );
        }
        (result, biggest_stack)
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
        let tmp: HashSet<Rd> = blocks
            .into_iter()
            .map(|x| {
                x.iter()
                    .map(|x| x.get_reads())
                    .flatten()
                    .collect::<Vec<Rd>>()
            })
            .flatten()
            .collect();

        tmp
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

    fn peepholer_run(peepholer: &PeepHoler, blocks: &mut Vec<AsmBasicBlock>) {
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
        AsmFunctionBuilder::peepholer_run(&peepholer, &mut blocks);

        let used_regs = AsmFunctionBuilder::get_used_regs(&blocks);
        let reg_allocator = LinearAllocator::new(
            self.ir_function,
            used_regs,
            self.stacksize as i64,
            self.liveness,
        );
        let stacksize = reg_allocator.get_stacksize();

        // do register allocation
        let mut biggest_addition = 0;
        let blocks: Vec<AsmBasicBlock> = blocks
            .into_iter()
            .map(|x| {
                let (block, addition) =
                    AsmFunctionBuilder::patch_ir_registers(&reg_allocator, x, stacksize as i64);
                biggest_addition = std::cmp::max(addition, biggest_addition);
                block
            })
            .collect();
        let stacksize = stacksize + biggest_addition as usize;

        // epilogues
        let mut blocks: Vec<AsmBasicBlock> = blocks
            .into_iter()
            .map(|x| AsmFunctionBuilder::add_epilogue(x, stacksize))
            .collect();

        // prolog
        blocks
            .first_mut()
            .expect("Totally empty function")
            .insert(0, AsmInstruction::Addi(Rd::Sp, Rd::Sp, -(stacksize as i64)));

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
