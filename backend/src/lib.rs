use std::fmt::Display;

use middleend::ir::IrProgram;

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
        todo!()
    }
}

type AsmBasicBlock = (String, Vec<AsmInstruction>);

pub struct AsmProgram {
    data: Vec<(String, Data)>,
    text: Vec<AsmBasicBlock>,
}

impl Default for AsmProgram {
    fn default() -> Self {
        Self {
            data: vec![],
            text: vec![],
        }
    }
}

pub fn asm_compile(ir_program: IrProgram) -> AsmProgram {
    let program = AsmProgram::default();

    program
}

pub fn emit_assembly(program: AsmProgram) -> String {
    let mut lines = vec![
        ".global _start".to_string(),
        "_start:".to_string(),
        "j main".to_string(),
    ];

    lines.append(&mut program.text.iter().flat_map(emit_basicblock).collect());

    lines.join("\n")
}

fn emit_basicblock(block: &AsmBasicBlock) -> Vec<String> {
    let (name, code) = block;
    let mut result = vec![name.clone() + ":"];
    result.append(&mut code.iter().map(|x| x.to_string()).collect());
    result
}
