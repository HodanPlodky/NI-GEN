pub mod emit;

use std::collections::HashMap;

use middleend::{
    inst::{BasicBlock, ImmI, Instruction, RegReg, TerminatorReg},
    ir::{Function, IrProgram},
};

type Data = Vec<u8>;

type Rd = usize;
type Imm = i64;
type Offset = i32;

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
    Lbu(Rd, Rd, Offset),
    Lhu(Rd, Rd, Offset),

    Sb(Rd, Rd, Offset),
    Sh(Rd, Rd, Offset),
    Sw(Rd, Rd, Offset),

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
    let start: AsmBasicBlock = ir_program
        .glob
        .iter()
        .flat_map(basic_instruction_selection)
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
    let result = function.blocks.into_iter().map(asm_basicblock).collect();
    AsmFunction {
        name: function.name,
        blocks: result,
    }
}

fn asm_basicblock(block: BasicBlock) -> AsmBasicBlock {
    block.iter().flat_map(basic_instruction_selection).collect()
}

fn basic_instruction_selection(inst: &Instruction) -> Vec<AsmInstruction> {
    let t: [usize; 7] = [5, 6, 7, 28, 29, 30, 31];
    let a: [usize; 8] = [10, 11, 12, 13, 14, 15, 16, 17];
    let reg = inst.id.2;
    match &inst.data {
        middleend::inst::InstructionType::Ldi(ImmI(imm)) => {
            vec![AsmInstruction::Addi(t[reg], 0, *imm)]
        }
        middleend::inst::InstructionType::Ldc(_) => todo!(),
        middleend::inst::InstructionType::Ld(_) => todo!(),
        middleend::inst::InstructionType::St(_) => todo!(),
        middleend::inst::InstructionType::Alloca(_) => todo!(),
        middleend::inst::InstructionType::Allocg(_) => todo!(),
        middleend::inst::InstructionType::Cpy(_) => todo!(),
        middleend::inst::InstructionType::Gep(_) => todo!(),
        middleend::inst::InstructionType::Add(RegReg(r1, r2)) => {
            vec![AsmInstruction::Add(t[reg], t[r1.2], t[r2.2])]
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
        middleend::inst::InstructionType::Ret(_) => todo!(),
        middleend::inst::InstructionType::Exit(_) => vec![],
        middleend::inst::InstructionType::Retr(TerminatorReg(reg)) => {
            vec![AsmInstruction::Addi(a[0], t[reg.2], 0), AsmInstruction::Ret]
        }
        middleend::inst::InstructionType::Jmp(_) => todo!(),
        middleend::inst::InstructionType::Branch(_) => todo!(),
        middleend::inst::InstructionType::Print(_) => todo!(),
        middleend::inst::InstructionType::Phi(_) => todo!(),
    }
}

enum ValueCell {
    Register(usize),
    StackOffset(usize),
}

struct AsmFunctionBuilder {
    stacksize: usize,
    blocks: Vec<AsmBasicBlock>,

    registers : HashMap<middleend::inst::Register, ValueCell>,
}

impl AsmFunctionBuilder {
    fn new() -> Self {
        Self {
            stacksize: 0,
            blocks: vec![],
            
            registers : HashMap::new(),
        }
    }

    fn create_block(&mut self) {
        self.blocks.push(AsmBasicBlock::new());
    }

    fn allocate_reg(&mut self) -> ValueCell {
        todo!()
    }

    fn add_instruction(&mut self) {

    }
}
