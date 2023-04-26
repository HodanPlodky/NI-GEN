use middleend::ir::IrProgram;

type Data = Vec<u8>;

type Register = usize;

struct AsmInstruction {
    Ldi(Register, i32),
    Addi(Register, Register, i32),

}

type AsmBasicBlock = Vec<AsmInstruction>;

pub struct AsmProgram {
    data : Vec<(String, Data)>,
    text : Vec<AsmBasicBlock>
}

pub fn asm_compile(ir_program : IrProgram) -> AsmProgram {
    let x = Data::new();
    todo!()
}
