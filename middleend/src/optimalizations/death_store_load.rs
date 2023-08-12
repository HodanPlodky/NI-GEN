use crate::{
    analysis::{
        const_mem::{ConstantMemoryAnalysis, MemoryPlace},
        dataflow::DataFlowAnalysis,
        lattice::FlatElem,
    },
    inst::{InstructionType, Reg},
    ir::Function,
};

pub fn remove_store_load(function: &mut Function) {
    // rewrite loads into just copies
    // if possible
    let mut const_analysis = ConstantMemoryAnalysis::new(function);
    let result = const_analysis.analyze();

    let used = function.get_used_regs();
    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        for inst_index in 0..bb.len() {
            match bb[inst_index].data {
                InstructionType::Ld(Reg(addr)) => {
                    let state = &result[bb_index][inst_index];
                    match state.get(&MemoryPlace(addr)) {
                        Some(FlatElem::Value(val)) => {
                            bb[inst_index].data = InstructionType::Mov(Reg(*val))
                        }
                        Some(_) | None => (),
                    }
                }
                _ => (),
            }
        }
    }

    //remove_unused_instruction(function);
}

fn remove_unused_instruction(function: &mut Function) {
    let used = function.get_used_regs();
    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        let mut inst_index = 0;
        while inst_index < bb.len() {
            if !used.contains(&bb[inst_index].id) {
                bb.remove(inst_index);
            } else {
                inst_index += 1;
            }
        }
    }
}
