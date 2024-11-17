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
use middleend::ir::{BasicBlock, Function, InstStore, IrProgram};
use peepholer::{MockDatabase, PeepHoler};

pub fn asm_compile(ir_program: IrProgram) -> AsmProgram {
    let mut start = asm_func(ir_program.glob, &ir_program.store);

    let text: Vec<AsmFunction> = ir_program
        .funcs
        .into_iter()
        .map(|x| asm_func(x.1, &ir_program.store))
        .collect();

    AsmProgram {
        start: std::mem::take(&mut start.blocks[0]),
        text,
        data: vec![],
    }
}

fn asm_func(function: Function, store: &InstStore) -> AsmFunction {
    let mut builder = AsmFunctionBuilder::new(function.name.clone(), &function, store);

    function
        .blocks
        .iter()
        .for_each(|x| asm_basicblock(x, &mut builder, store));

    let database = MockDatabase {};
    let peephole = PeepHoler::new(&database);
    builder.build(peephole)
}

fn asm_basicblock(
    block: &BasicBlock,
    builder: &mut AsmFunctionBuilder,
    store: &InstStore,
) {
    builder.actual_bb = builder.create_block();
    block.iter().for_each(|x| {
        basic_instruction_selection(store.get(*x), *x, builder, store)
    })
}
