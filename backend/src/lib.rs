pub mod emit;

use std::collections::HashMap;

use middleend::{
    inst::{
        BasicBlock, ImmI, Instruction, Reg, RegReg, SymRegs, TerminatorBranch, TerminatorJump,
        TerminatorReg,
    },
    ir::{Function, IrProgram},
};

type Data = Vec<u8>;

type Rd = usize;
type Imm = i64;
type Offset = i64;

#[derive(Clone, PartialEq, Eq)]
enum AsmInstruction {
    Lui(Rd, Imm),
    Auipc(Rd, Imm),

    Jal(Rd, Offset, String),
    Jalr(Rd, Rd, Offset),

    Beq(Rd, Rd, Offset, String),
    Bne(Rd, Rd, Offset, String),
    Blt(Rd, Rd, Offset, String),
    Bge(Rd, Rd, Offset, String),
    Bltu(Rd, Rd, Offset, String),
    Bgeu(Rd, Rd, Offset, String),

    Lb(Rd, Rd, Offset),
    Lh(Rd, Rd, Offset),
    Lw(Rd, Rd, Offset),
    Ld(Rd, Rd, Offset),
    Lbu(Rd, Rd, Offset),
    Lhu(Rd, Rd, Offset),

    Sb(Rd, Rd, Offset),
    Sh(Rd, Rd, Offset),
    Sw(Rd, Rd, Offset),
    Sd(Rd, Rd, Offset),

    Addi(Rd, Rd, Imm),
    Slti(Rd, Rd, Imm),
    Sltiu(Rd, Rd, Imm),
    Xori(Rd, Rd, Imm),
    Ori(Rd, Rd, Imm),
    Andi(Rd, Rd, Imm),
    Slli(Rd, Rd, Imm),
    Srli(Rd, Rd, Imm),
    Srai(Rd, Rd, Imm),

    Add(Rd, Rd, Rd),
    Sub(Rd, Rd, Rd),
    Sll(Rd, Rd, Rd),
    Srl(Rd, Rd, Rd),
    Slt(Rd, Rd, Rd),
    Sltu(Rd, Rd, Rd),
    Xor(Rd, Rd, Rd),
    Or(Rd, Rd, Rd),
    And(Rd, Rd, Rd),
    Sra(Rd, Rd, Rd),

    // pseudo instructions
    Call(String),
    Ret,
}

impl AsmInstruction {
    fn size(&self) -> usize {
        match self {
            AsmInstruction::Call(_) => 4,
            AsmInstruction::Add(_, _, _) => 4,
            _ => 4,
        }
    }
}

type AsmBasicBlock = Vec<AsmInstruction>;

struct AsmFunction {
    name: String,
    blocks: Vec<AsmBasicBlock>,
}

pub struct AsmProgram {
    data: Vec<(String, Data)>,
    start: AsmBasicBlock,
    text: Vec<AsmFunction>,
}

impl Default for AsmProgram {
    fn default() -> Self {
        Self {
            data: vec![],
            start: vec![],
            text: vec![],
        }
    }
}

pub fn asm_compile(ir_program: IrProgram) -> AsmProgram {
    let mut startbuilder = AsmFunctionBuilder::new("global".to_string());
    startbuilder.create_block();
    ir_program
        .glob
        .iter()
        .for_each(|x| basic_instruction_selection(x, &mut startbuilder));

    let mut start = startbuilder.build();

    let text: Vec<AsmFunction> = ir_program
        .funcs
        .into_iter()
        .map(|x| asm_func(x.1))
        .collect();

    AsmProgram {
        start: std::mem::take(&mut start.blocks[0]),
        text,
        data: vec![],
    }
}

fn asm_func(function: Function) -> AsmFunction {
    let mut builder = AsmFunctionBuilder::new(function.name);

    function
        .blocks
        .into_iter()
        .for_each(|x| asm_basicblock(x, &mut builder));

    builder.build()
}

fn asm_basicblock(block: BasicBlock, builder: &mut AsmFunctionBuilder) {
    builder.actual_bb = builder.create_block();
    block
        .iter()
        .for_each(|x| basic_instruction_selection(x, builder))
}

fn basic_instruction_selection(inst: &Instruction, builder: &mut AsmFunctionBuilder) {
    let a: [usize; 8] = [10, 11, 12, 13, 14, 15, 16, 17];
    let reg = inst.id;
    match &inst.data {
        middleend::inst::InstructionType::Ldi(ImmI(imm)) => {
            builder.allocate_reg(reg);
            let out = builder.get_reg(reg);
            builder.add_instruction(AsmInstruction::Addi(out, 0, *imm));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Ldc(_) => todo!(),
        middleend::inst::InstructionType::Ld(Reg(rs1)) => {
            builder.allocate_reg(reg);
            let out = builder.get_reg(reg);
            match builder.get_offset(*rs1) {
                Some(offset) => {
                    builder.add_instruction(AsmInstruction::Ld(out, 2, offset));
                }
                None => todo!(),
            }
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::St(RegReg(rs1, rs2)) => {
            let input = builder.get_reg(*rs2);
            match builder.get_offset(*rs1) {
                Some(offset) => {
                    builder.add_instruction(AsmInstruction::Sd(input, 2, offset));
                }
                None => todo!(),
            }
        }
        middleend::inst::InstructionType::Alloca(ImmI(size)) => {
            let offset = builder.allocate_stack(*size);
            builder.store_offset(reg, offset);
        }
        middleend::inst::InstructionType::Allocg(_) => todo!(),
        middleend::inst::InstructionType::Cpy(_) => todo!(),
        middleend::inst::InstructionType::Gep(_) => todo!(),
        middleend::inst::InstructionType::Add(RegReg(r1, r2)) => {
            builder.allocate_reg(reg);
            let out = builder.get_reg(reg);
            let asm_r1 = builder.get_reg(*r1);
            let asm_r2 = builder.get_reg(*r2);
            builder.add_instruction(AsmInstruction::Add(out, asm_r1, asm_r2));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Sub(RegReg(rs1, rs2)) => {
            builder.allocate_reg(reg);
            let out = builder.get_reg(reg);
            let asm_r1 = builder.get_reg(*rs1);
            let asm_r2 = builder.get_reg(*rs2);
            builder.add_instruction(AsmInstruction::Sub(out, asm_r1, asm_r2));
            builder.store_reg(reg, out);
            builder.release_temp();
        },
        middleend::inst::InstructionType::Mul(_) => todo!(),
        middleend::inst::InstructionType::Div(_) => todo!(),
        middleend::inst::InstructionType::Mod(_) => todo!(),
        middleend::inst::InstructionType::Shr(_) => todo!(),
        middleend::inst::InstructionType::Shl(_) => todo!(),
        middleend::inst::InstructionType::And(_) => todo!(),
        middleend::inst::InstructionType::Or(_) => todo!(),
        middleend::inst::InstructionType::Xor(_) => todo!(),
        middleend::inst::InstructionType::Neg(_) => todo!(),
        middleend::inst::InstructionType::Le(_) => todo!(),
        middleend::inst::InstructionType::Lt(RegReg(rs1, rs2)) => {
            builder.allocate_reg(reg);
            let out = builder.get_reg(reg);
            let asm1 = builder.get_reg(*rs1);
            let asm2 = builder.get_reg(*rs2);
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
                let src = builder.get_reg(regs[i]);
                builder.add_instruction(AsmInstruction::Addi(a[i], src, 0));
                builder.release_temp();
            }
            let offset = builder.force_store(1);
            builder.add_instruction(AsmInstruction::Call(sym.clone()));
            builder.add_instruction(AsmInstruction::Ld(1, 2, offset));
            builder.allocate_reg(reg);
            let out = builder.get_reg(reg);
            builder.add_instruction(AsmInstruction::Addi(out, a[0], 0));
            builder.store_reg(reg, out);
            builder.release_temp();
        }
        middleend::inst::InstructionType::Arg(ImmI(imm)) => {
            builder.allocate_reg(reg);
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
        middleend::inst::InstructionType::Jmp(TerminatorJump(bb_index)) => {
            builder.add_instruction(AsmInstruction::Jal(0, *bb_index as i64, builder.name.clone()))
        }
        middleend::inst::InstructionType::Branch(TerminatorBranch(reg, _, false_bb)) => {
            let input = builder.get_reg(*reg);
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

#[derive(Clone, Copy)]
enum ValueCell {
    Register(usize),
    StackOffset(i64),
}

type OffsetEnv = HashMap<middleend::inst::Register, Offset>;

struct AsmFunctionBuilder {
    name: String,
    stacksize: usize,
    actual_bb: usize,
    blocks: Vec<AsmBasicBlock>,

    registers: HashMap<middleend::inst::Register, ValueCell>,
    freeowned: Vec<usize>,
    freetemp: Vec<usize>,
    offsets: OffsetEnv,
}

impl AsmFunctionBuilder {
    fn new(name: String) -> Self {
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

    fn bb_size(block: &AsmBasicBlock) -> usize {
        block.len() * 4
        //block.iter().map(|x| x.size()).sum()
    }

    fn patch_jumps(offsets: &Vec<usize>, block: AsmBasicBlock) -> AsmBasicBlock {
        let mut block = block;
        match block.last() {
            Some(AsmInstruction::Jal(rd, offset, name)) => {
                let rd = *rd;
                let offset = *offset;
                let name = name.clone();
                block.pop();
                block.push(AsmInstruction::Jal(
                    rd,
                    offsets[offset as usize] as i64,
                    name,
                ));
                block
            }
            Some(AsmInstruction::Beq(rd1, rd2, offset, name)) => {
                let rd1 = *rd1;
                let rd2 = *rd2;
                let offset = *offset;
                let name = name.clone();
                block.pop();
                block.push(AsmInstruction::Beq(
                    rd1,
                    rd2,
                    offsets[offset as usize] as i64,
                    name,
                ));
                block
            }
            Some(AsmInstruction::Bne(rd1, rd2, offset, name)) => {
                let rd1 = *rd1;
                let rd2 = *rd2;
                let offset = *offset;
                let name = name.clone();
                block.pop();
                block.push(AsmInstruction::Bne(
                    rd1,
                    rd2,
                    offsets[offset as usize] as i64,
                    name,
                ));
                block
            }
            _ => block,
        }
    }

    fn build(self) -> AsmFunction {
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

    fn create_block(&mut self) -> usize {
        self.blocks.push(AsmBasicBlock::new());
        self.blocks.len() - 1
    }

    fn allocate_reg(&mut self, reg: middleend::inst::Register) -> ValueCell {
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

    fn get_reg(&mut self, reg: middleend::inst::Register) -> usize {
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

    fn store_reg(&mut self, reg: middleend::inst::Register, tmpreg: usize) {
        match self.registers.get(&reg) {
            Some(ValueCell::Register(_)) => (),
            Some(ValueCell::StackOffset(offset)) => {
                self.add_instruction(AsmInstruction::Sd(tmpreg, 2, *offset))
            }
            None => unreachable!(),
        }
    }

    fn force_store(&mut self, reg: usize) -> Offset {
        let offset = self.stacksize;
        self.stacksize += 8;
        self.add_instruction(AsmInstruction::Sd(reg, 2, offset as i64));
        offset as i64
    }

    fn allocate_stack(&mut self, size: i64) -> Offset {
        let offset = self.stacksize;
        self.stacksize += size as usize;
        offset as i64
    }

    fn store_offset(&mut self, reg: middleend::inst::Register, offset: Offset) {
        self.offsets.insert(reg, offset);
    }

    fn get_offset(&mut self, reg: middleend::inst::Register) -> Option<Offset> {
        self.offsets.get(&reg).copied()
    }

    fn release_temp(&mut self) {
        self.freetemp = vec![29, 30, 31];
    }

    fn add_instruction(&mut self, inst: AsmInstruction) {
        self.blocks.last_mut().unwrap().push(inst);
    }
}
