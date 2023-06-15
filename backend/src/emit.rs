use std::fmt::Display;

use crate::{AsmInstruction, AsmProgram, AsmBasicBlock, AsmFunction};


impl Display for AsmInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsmInstruction::Lui(rd, imm) => {
                write!(f, "lui {}, {}", rd, imm)
            },
            AsmInstruction::Auipc(rd, imm) => write!(f, "auipc {}, {}", rd, imm),
            AsmInstruction::Jal(rd, imm) => write!(f, "jal {}, {}", rd, imm),
            AsmInstruction::Jalr(x, y, z) => write!(f, "jalr x{}, x{}, x{}", x, y, z),
            AsmInstruction::Beq(x, y, z) => write!(f, "beq x{}, x{}, x{}", x, y, z),
            AsmInstruction::Bne(x, y, z) => write!(f, "bne x{}, x{}, x{}", x, y, z),
            AsmInstruction::Blt(x, y, z) => write!(f, "blt x{}, x{}, x{}", x, y, z),
            AsmInstruction::Bge(x, y, z) => write!(f, "bge x{}, x{}, x{}", x, y, z),
            AsmInstruction::Bltu(x, y, z) => write!(f, "bltu x{}, x{}, x{}", x, y, z),
            AsmInstruction::Bgeu(x, y, z) => write!(f, "bgeu x{}, x{}, x{}", x, y, z),
            AsmInstruction::Lb(x, y, z) => write!(f, "lb x{}, x{}, x{}", x, y, z),
            AsmInstruction::Lh(x, y, z) => write!(f, "lh x{}, x{}, x{}", x, y, z),
            AsmInstruction::Lw(x, y, z) => write!(f, "lw x{}, x{}, x{}", x, y, z),
            AsmInstruction::Ld(x, y, offset) => write!(f, "ld x{}, {}(x{})", x, offset, y),
            AsmInstruction::Lbu(x, y, z) => write!(f, "lbu x{}, x{}, x{}", x, y, z),
            AsmInstruction::Lhu(x, y, z) => write!(f, "lhu x{}, x{}, x{}", x, y, z),
            AsmInstruction::Sb(x, y, z) => write!(f, "sb x{}, x{}, x{}", x, y, z),
            AsmInstruction::Sh(x, y, z) => write!(f, "sh x{}, x{}, x{}", x, y, z),
            AsmInstruction::Sw(x, y, z) => write!(f, "sw x{}, x{}, x{}", x, y, z),
            AsmInstruction::Sd(x, y, offset) => write!(f, "sd x{}, {}(x{})", x, offset, y),
            AsmInstruction::Addi(x, y, z) => write!(f, "addi x{}, x{}, {}", x, y, z),
            AsmInstruction::Slti(x, y, z) => write!(f, "slti x{}, x{}, x{}", x, y, z),
            AsmInstruction::Sltiu(x, y, z) => write!(f, "sltiu x{}, x{}, x{}", x, y, z),
            AsmInstruction::Xori(x, y, z) => write!(f, "xori x{}, x{}, x{}", x, y, z),
            AsmInstruction::Ori(x, y, z) => write!(f, "ori x{}, x{}, x{}", x, y, z),
            AsmInstruction::Andi(x, y, z) => write!(f, "andi x{}, x{}, {}", x, y, z),
            AsmInstruction::Slli(x, y, z) => write!(f, "slli x{}, x{}, x{}", x, y, z),
            AsmInstruction::Srli(x, y, z) => write!(f, "srli x{}, x{}, x{}", x, y, z),
            AsmInstruction::Srai(x, y, z) => write!(f, "srai x{}, x{}, x{}", x, y, z),
            AsmInstruction::Add(x, y, z) => write!(f, "add x{}, x{}, x{}", x, y, z),
            AsmInstruction::Sub(x, y, z) => write!(f, "sub x{}, x{}, x{}", x, y, z),
            AsmInstruction::Sll(x, y, z) => write!(f, "sll x{}, x{}, x{}", x, y, z),
            AsmInstruction::Srl(x, y, z) => write!(f, "srl x{}, x{}, x{}", x, y, z),
            AsmInstruction::Slt(x, y, z) => write!(f, "slt x{}, x{}, x{}", x, y, z),
            AsmInstruction::Sltu(x, y, z) => write!(f, "sltu x{}, x{}, x{}", x, y, z),
            AsmInstruction::Xor(x, y, z) => write!(f, "xor x{}, x{}, x{}", x, y, z),
            AsmInstruction::Or(x, y, z) => write!(f, "or x{}, x{}, x{}", x, y, z),
            AsmInstruction::And(x, y, z) => write!(f, "and x{}, x{}, x{}", x, y, z),
            AsmInstruction::Sra(x, y, z) => write!(f, "sra x{}, x{}, x{}", x, y, z),
            AsmInstruction::Call(imm) => write!(f, "call {}", imm),
            AsmInstruction::Ret => write!(f, "ret"),
        }
    }
}


pub fn emit_assembly(program: AsmProgram) -> String {
    let mut lines = vec![".global _start".to_string(), "_start:".to_string()];

    // appending any prologue
    lines.append(&mut emit_basicblock(program.start));

    lines.push("    call main".to_string());

    // epilogue
    lines.push("    addi a7, zero, 93".to_string());
    lines.push("    ecall".to_string());

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
    block.iter().map(|x| "    ".to_string() + x.to_string().as_str()).collect()
}
