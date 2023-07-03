pub mod emit;
mod fn_builder;
mod inst_selection;
mod insts;
mod peepholer;
mod register_alloc;

use fn_builder::AsmFunctionBuilder;
use inst_selection::basic_instruction_selection;
use insts::AsmInstruction;
use middleend::{
    inst::BasicBlock,
    ir::{Function, IrProgram},
};
use peepholer::{MockDatabase, PeepHoler};
use register_alloc::{LinearAllocator, NaiveAllocator};

type Data = Vec<u8>;

pub type AsmBasicBlock = Vec<AsmInstruction>;

pub struct AsmFunction {
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
    let mut start = asm_func(ir_program.glob);

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
    let reg_alloc = LinearAllocator::new(&function);
    let mut builder = AsmFunctionBuilder::new(function.name, &reg_alloc);

    function
        .blocks
        .into_iter()
        .for_each(|x| asm_basicblock(x, &mut builder));

    let database = MockDatabase {};
    let peephole = PeepHoler::new(&database);
    builder.build(peephole)
}

fn asm_basicblock(block: BasicBlock, builder: &mut AsmFunctionBuilder) {
    builder.actual_bb = builder.create_block();
    block
        .iter()
        .for_each(|x| basic_instruction_selection(x, builder))
}
