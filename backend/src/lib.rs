mod backend_ir;
pub mod emit;
mod fn_builder;
mod inst_selection;
mod insts;
mod peepholer;
mod register_alloc;

use backend_ir::{AsmFunction, AsmProgram};
use fn_builder::AsmFunctionBuilder;
use inst_selection::basic_instruction_selection;
use middleend::{
    ir::BasicBlock,
    ir::{Function, IrProgram},
};
use peepholer::{MockDatabase, PeepHoler};

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
    let mut builder = AsmFunctionBuilder::new(function.name.clone(), &function);

    function
        .blocks
        .iter()
        .for_each(|x| asm_basicblock(x, &mut builder, &function));

    let database = MockDatabase {};
    let peephole = PeepHoler::new(&database);
    builder.build(peephole)
}

fn asm_basicblock(block: &BasicBlock, builder: &mut AsmFunctionBuilder, function: &Function) {
    builder.actual_bb = builder.create_block();
    block.iter().zip(0..).for_each(|(x, inst_index)| {
        basic_instruction_selection(x, (x.id.0, x.id.1, inst_index), builder, function)
    })
}
