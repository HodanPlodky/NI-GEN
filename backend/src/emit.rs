use std::fmt::Display;

use crate::{AsmInstruction, AsmProgram};


impl Display for AsmInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsmInstruction::Lui(rd, imm) => {
                write!(f, "lui {}, {}", rd, imm)
            },
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
            AsmInstruction::Call(imm) => write!(f, "call {}", imm),
            AsmInstruction::Ret => write!(f, "ret"),
        }
    }
}


pub fn emit_assembly(program: AsmProgram) -> String {
    let mut lines = vec![".global _start".to_string(), "_start:".to_string()];

    // appending any prologue
    lines.append(&mut emit_basicblock(program.start));

    lines.push("call main".to_string());

    // epilogue
    lines.push("addi a7, zero, 93".to_string());
    lines.push("ecall".to_string());

    // main logic
    lines.append(&mut program.text.into_iter().flat_map(emit_function).collect());

    lines.join("\n")
}

fn emit_function(function: AsmFunction) -> Vec<String> {
    let mut result = vec![function.name + ":"];
    let mut code = function
        .blocks
        .into_iter()
        .flat_map(emit_basicblock)
        .collect();
    result.append(&mut code);

    result
}

fn emit_basicblock(block: AsmBasicBlock) -> Vec<String> {
    block.iter().map(|x| x.to_string()).collect()
}
