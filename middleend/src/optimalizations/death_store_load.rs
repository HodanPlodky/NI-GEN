use std::collections::{HashMap, HashSet};

use crate::{
    analysis::{
        anderson::{AndersenAnalysis, Cell},
        const_mem::{ConstantMemoryAnalysis, MemoryPlace},
        dataflow::DataFlowAnalysis,
        lattice::FlatElem,
    },
    inst::{InstructionType, Reg, RegReg},
    ir::{Function, RegType, Register},
};

pub fn remove_store_load(function: &mut Function) -> bool {
    // rewrite loads into just copies
    // if possible
    let mut const_analysis = ConstantMemoryAnalysis::new(function);
    let result = const_analysis.analyze();

    let mut change = false;

    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        for inst_index in 0..bb.len() {
            match bb[inst_index].data {
                InstructionType::Ld(Reg(addr)) => {
                    let state = &result[bb_index][inst_index];
                    match state.get(&MemoryPlace(addr)) {
                        Some(FlatElem::Value(val)) => {
                            change = true;
                            bb[inst_index].data = InstructionType::Mov(Reg(*val))
                        }
                        Some(_) | None => (),
                    }
                }
                _ => (),
            }
        }
    }

    change |= remove_unused_stores(function);

    change |= remove_movs(function);

    change |= remove_unused_instruction(function);

    change
}

fn remove_unused_instruction(function: &mut Function) -> bool {
    let mut change = false;
    let used = function.get_used_regs();
    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        let mut inst_index = 0;
        while inst_index < bb.len() {
            if !used.contains(&bb[inst_index].id) && bb[inst_index].reg_type != RegType::Void {
                bb.remove(inst_index);
                change = true;
            } else {
                inst_index += 1;
            }
        }
    }

    change
}

fn remove_unused_stores(function: &mut Function) -> bool {
    let mut change = false;
    let mut loads: Vec<Register> = vec![];
    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        for inst_index in 0..bb.len() {
            match bb[inst_index].data {
                InstructionType::Ld(Reg(addr)) => {
                    loads.push(addr);
                }
                _ => (),
            }
        }
    }

    let mut pointer_analysis = AndersenAnalysis::new(function);
    let result = pointer_analysis.analyze();

    let loads: HashSet<Cell> = loads
        .iter()
        .map(|x| match result.get(x) {
            Some(cells) => cells.clone(),
            None => HashSet::new(),
        })
        .flatten()
        .collect();

    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        let mut inst_index = 0;
        while inst_index < bb.len() {
            match bb[inst_index].data {
                InstructionType::St(RegReg(addr, _)) => {
                    let cells = match result.get(&addr) {
                        Some(cells) => cells.clone(),
                        None => HashSet::new(),
                    };

                    if cells.is_disjoint(&loads) && !cells.contains(&Cell::Volatile) {
                        change = true;
                        bb.remove(inst_index);
                    } else {
                        inst_index += 1;
                    }
                }
                _ => inst_index += 1,
            }
        }
    }

    change
}

fn remove_movs(function: &mut Function) -> bool {
    let mut change = false;
    let mut renames: HashMap<Register, Register> = HashMap::new();
    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        let mut inst_index = 0;
        while inst_index < bb.len() {
            match bb[inst_index].data {
                InstructionType::Mov(Reg(reg)) => {
                    renames.insert(bb[inst_index].id, reg);
                    bb.remove(inst_index);
                    change = true;
                }
                _ => {
                    bb[inst_index].data.rename_regs(&renames);
                    inst_index += 1
                }
            }
        }
    }
    change
}
