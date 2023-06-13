use std::{collections::HashMap, fmt::Display};

use middleend::{
    inst::Instruction,
    ir::{Function, IrProgram},
};

type Data = Vec<u8>;

type Rd = usize;
type Imm = i32;
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
}

impl Display for AsmInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsmInstruction::Lui(rd, imm) => write!(f, "lui {}, {}", rd, imm),
            AsmInstruction::Auipc(rd, imm) => write!(f, "auipc {}, {}", rd, imm),
            AsmInstruction::Jal(rd, imm) => write!(f, "jal {}, {}", rd, imm),
            AsmInstruction::Jalr(x, y, z) => write!(f, "jalr {}, {}, {}", x, y, z),
            AsmInstruction::Beq(x, y, z) => write!(f, "beq {}, {}, {}", x, y, z),
            AsmInstruction::Bne(x, y, z) => write!(f, "bne {}, {}, {}", x, y, z),
            AsmInstruction::Blt(x, y, z) => write!(f, "blt {}, {}, {}", x, y, z),
            AsmInstruction::Bge(x, y, z) => write!(f, "bge {}, {}, {}", x, y, z),
            AsmInstruction::Bltu(x, y, z) => write!(f, "bltu {}, {}, {}", x, y, z),
            AsmInstruction::Bgeu(x, y, z) => write!(f, "bgeu {}, {}, {}", x, y, z),
            AsmInstruction::Lb(x, y, z) => write!(f, "lb {}, {}, {}", x, y, z),
            AsmInstruction::Lh(x, y, z) => write!(f, "lh {}, {}, {}", x, y, z),
            AsmInstruction::Lw(x, y, z) => write!(f, "lw {}, {}, {}", x, y, z),
            AsmInstruction::Lbu(x, y, z) => write!(f, "lbu {}, {}, {}", x, y, z),
            AsmInstruction::Lhu(x, y, z) => write!(f, "lhu {}, {}, {}", x, y, z),
            AsmInstruction::Sb(x, y, z) => write!(f, "sb {}, {}, {}", x, y, z),
            AsmInstruction::Sh(x, y, z) => write!(f, "sh {}, {}, {}", x, y, z),
            AsmInstruction::Sw(x, y, z) => write!(f, "sw {}, {}, {}", x, y, z),
            AsmInstruction::Addi(x, y, z) => write!(f, "addi {}, {}, {}", x, y, z),
            AsmInstruction::Slti(x, y, z) => write!(f, "slti {}, {}, {}", x, y, z),
            AsmInstruction::Sltiu(x, y, z) => write!(f, "sltiu {}, {}, {}", x, y, z),
            AsmInstruction::Xori(x, y, z) => write!(f, "xori {}, {}, {}", x, y, z),
            AsmInstruction::Ori(x, y, z) => write!(f, "ori {}, {}, {}", x, y, z),
            AsmInstruction::Andi(x, y, z) => write!(f, "andi {}, {}, {}", x, y, z),
            AsmInstruction::Slli(x, y, z) => write!(f, "slli {}, {}, {}", x, y, z),
            AsmInstruction::Srli(x, y, z) => write!(f, "srli {}, {}, {}", x, y, z),
            AsmInstruction::Srai(x, y, z) => write!(f, "srai {}, {}, {}", x, y, z),
            AsmInstruction::Add(x, y, z) => write!(f, "add {}, {}, {}", x, y, z),
            AsmInstruction::Sub(x, y, z) => write!(f, "sub {}, {}, {}", x, y, z),
            AsmInstruction::Sll(x, y, z) => write!(f, "sll {}, {}, {}", x, y, z),
            AsmInstruction::Srl(x, y, z) => write!(f, "srl {}, {}, {}", x, y, z),
            AsmInstruction::Slt(x, y, z) => write!(f, "slt {}, {}, {}", x, y, z),
            AsmInstruction::Sltu(x, y, z) => write!(f, "sltu {}, {}, {}", x, y, z),
            AsmInstruction::Xor(x, y, z) => write!(f, "xor {}, {}, {}", x, y, z),
            AsmInstruction::Or(x, y, z) => write!(f, "or {}, {}, {}", x, y, z),
            AsmInstruction::And(x, y, z) => write!(f, "and {}, {}, {}", x, y, z),
            AsmInstruction::Sra(x, y, z) => write!(f, "sra {}, {}, {}", x, y, z),
        }
    }
}

type AsmBasicBlock = Vec<AsmInstruction>;

pub struct AsmProgram {
    data: Vec<(String, Data)>,
    text: HashMap<String, AsmBasicBlock>,
}

impl Default for AsmProgram {
    fn default() -> Self {
        Self {
            data: vec![],
            text: HashMap::new(),
        }
    }
}

pub fn asm_compile(ir_program: IrProgram) -> AsmProgram {
    let program = AsmProgram::default();

    let prolog: AsmBasicBlock = ir_program
        .glob
        .iter()
        .flat_map(instruction_selection)
        .collect();

    let text: Vec<(String, AsmBasicBlock)> = ir_program
        .funcs
        .into_iter()
        .flat_map(|x| asm_func(x.0, x.1))
        .collect();

    program
}

fn asm_func(name: String, function: Function) -> Vec<(String, AsmBasicBlock)> {
    todo!() 
}

fn instruction_selection(inst: &Instruction) -> Vec<AsmInstruction> {
    todo!()
}

pub fn emit_assembly(program: AsmProgram) -> String {
    let mut lines = vec![
        ".global _start".to_string(),
        "_start:".to_string(),
        "call main".to_string(),
    ];

    lines.append(
        &mut program
            .text
            .iter()
            .flat_map(|x| emit_basicblock(x.0, x.1))
            .collect(),
    );

    lines.join("\n")
}

fn emit_basicblock(name: &str, block: &AsmBasicBlock) -> Vec<String> {
    let mut result = vec![name.to_string() + ":"];
    result.append(&mut block.iter().map(|x| x.to_string()).collect());
    result
}
