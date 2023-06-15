pub mod emit;

use std::collections::HashMap;

use middleend::{
    inst::{BasicBlock, ImmI, Instruction, RegReg, TerminatorReg},
    ir::{Function, IrProgram},
};

type Data = Vec<u8>;

type Rd = usize;
type Imm = i64;
type Offset = i64;

enum AsmInstruction {
    Lui(Rd, Imm),
    Auipc(Rd, Imm),

    Jal(Rd, Offset),
    Jalr(Rd, Rd, Offset),

    Beq(Rd, Rd, Offset),
    Bne(Rd, Rd, Offset),
    Blt(Rd, Rd, Offset),
    Bge(Rd, Rd, Offset),
    Bltu(Rd, Rd, Offset),
    Bgeu(Rd, Rd, Offset),

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
    Call(Imm),
    Ret,
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
    let mut startbuilder = AsmFunctionBuilder::new();
    let start: AsmBasicBlock = ir_program
        .glob
        .iter()
        .flat_map(|x| basic_instruction_selection(x, &mut startbuilder))
        .collect();

    let text: Vec<AsmFunction> = ir_program
        .funcs
        .into_iter()
        .map(|x| asm_func(x.1))
        .collect();

    AsmProgram {
        start,
        text,
        data: vec![],
    }
}

fn asm_func(function: Function) -> AsmFunction {
    let mut builder = AsmFunctionBuilder::new();
    let result = function
        .blocks
        .into_iter()
        .map(|x| asm_basicblock(x, &mut builder))
        .collect();
    AsmFunction {
        name: function.name,
        blocks: result,
    }
}

fn asm_basicblock(block: BasicBlock, builder: &mut AsmFunctionBuilder) -> AsmBasicBlock {
    block
        .iter()
        .flat_map(|x| basic_instruction_selection(x, builder))
        .collect()
}

fn basic_instruction_selection(
    inst: &Instruction,
    builder: &mut AsmFunctionBuilder,
) -> Vec<AsmInstruction> {
    let a: [usize; 8] = [10, 11, 12, 13, 14, 15, 16, 17];
    let reg = inst.id;
    match &inst.data {
        middleend::inst::InstructionType::Ldi(ImmI(imm)) => {
            builder.allocate_reg(reg);
            let (mut result, out) = builder.get_reg(reg);
            result.push(AsmInstruction::Addi(out, 0, *imm));
            result.append(&mut builder.store_reg(reg, out));
            builder.release_temp();
            result
        }
        middleend::inst::InstructionType::Ldc(_) => todo!(),
        middleend::inst::InstructionType::Ld(_) => todo!(),
        middleend::inst::InstructionType::St(_) => todo!(),
        middleend::inst::InstructionType::Alloca(_) => todo!(),
        middleend::inst::InstructionType::Allocg(_) => todo!(),
        middleend::inst::InstructionType::Cpy(_) => todo!(),
        middleend::inst::InstructionType::Gep(_) => todo!(),
        middleend::inst::InstructionType::Add(RegReg(r1, r2)) => {
            builder.allocate_reg(reg);
            let (mut result, out) = builder.get_reg(reg);
            let (mut instsr1, asm_r1) = builder.get_reg(*r1);
            let (mut instsr2, asm_r2) = builder.get_reg(*r2);
            result.append(&mut instsr1);
            result.append(&mut instsr2);
            result.push(AsmInstruction::Add(out, asm_r1, asm_r2));
            result.append(&mut builder.store_reg(reg, out));
            builder.release_temp();
            result
        }
        middleend::inst::InstructionType::Sub(_) => todo!(),
        middleend::inst::InstructionType::Mul(_) => todo!(),
        middleend::inst::InstructionType::Div(_) => todo!(),
        middleend::inst::InstructionType::Mod(_) => todo!(),
        middleend::inst::InstructionType::Shr(_) => todo!(),
        middleend::inst::InstructionType::Shl(_) => todo!(),
        middleend::inst::InstructionType::And(_) => todo!(),
        middleend::inst::InstructionType::Or(_) => todo!(),
        middleend::inst::InstructionType::Xor(_) => todo!(),
        middleend::inst::InstructionType::Neg(_) => todo!(),
        middleend::inst::InstructionType::Lt(_) => todo!(),
        middleend::inst::InstructionType::Le(_) => todo!(),
        middleend::inst::InstructionType::Gt(_) => todo!(),
        middleend::inst::InstructionType::Ge(_) => todo!(),
        middleend::inst::InstructionType::Eql(_) => todo!(),
        middleend::inst::InstructionType::Fun(_) => todo!(),
        middleend::inst::InstructionType::Call(_) => todo!(),
        middleend::inst::InstructionType::Arg(_) => todo!(),
        middleend::inst::InstructionType::Ret(_) => vec![AsmInstruction::Ret],
        middleend::inst::InstructionType::Exit(_) => vec![],
        middleend::inst::InstructionType::Retr(TerminatorReg(reg)) => {
            builder.get_reg(*reg);
            let (mut result, out) = builder.get_reg(*reg);
            result.push(AsmInstruction::Addi(a[0], out, 0));
            result.push(AsmInstruction::Ret);
            builder.release_temp();
            result
        }
        middleend::inst::InstructionType::Jmp(_) => todo!(),
        middleend::inst::InstructionType::Branch(_) => todo!(),
        middleend::inst::InstructionType::Print(_) => todo!(),
        middleend::inst::InstructionType::Phi(_) => todo!(),
    }
}

#[derive(Clone, Copy)]
enum ValueCell {
    Register(usize),
    StackOffset(i64),
}

struct AsmFunctionBuilder {
    stacksize: usize,
    blocks: Vec<AsmBasicBlock>,

    registers: HashMap<middleend::inst::Register, ValueCell>,
    freeowned: Vec<usize>,
    freetemp: Vec<usize>,
}

impl AsmFunctionBuilder {
    fn new() -> Self {
        Self {
            stacksize: 0,
            blocks: vec![],
            freeowned: vec![5, 6, 7, 28],
            freetemp: vec![29, 30, 31],

            registers: HashMap::new(),
        }
    }

    fn create_block(&mut self) {
        self.blocks.push(AsmBasicBlock::new());
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

    fn get_reg(&mut self, reg: middleend::inst::Register) -> (Vec<AsmInstruction>, usize) {
        match self.registers.get(&reg) {
            Some(ValueCell::Register(reg)) => (vec![], *reg),
            Some(ValueCell::StackOffset(offset)) => {
                let target = self.freetemp.pop().unwrap().clone();
                (vec![AsmInstruction::Ld(target, 2, *offset)], target)
            }
            None => unreachable!(),
        }
    }

    fn store_reg(&mut self, reg: middleend::inst::Register, tmpreg: usize) -> Vec<AsmInstruction> {
        match self.registers.get(&reg) {
            Some(ValueCell::Register(_)) => vec![],
            Some(ValueCell::StackOffset(offset)) => vec![AsmInstruction::Sd(tmpreg, 2, *offset)],
            None => unreachable!(),
        }
    }

    fn release_temp(&mut self) {
        self.freetemp = vec![29, 30, 31];
    }

    fn add_instruction(&mut self, inst : AsmInstruction) {
        self.blocks.last_mut().unwrap().push(inst);
    }
}
