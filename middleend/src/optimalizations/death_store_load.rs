use std::collections::{HashMap, HashSet};

use crate::{
    analysis::{
        anderson::{AndersenAnalysis, Cell},
        const_mem::{ConstantMemoryAnalysis, MemoryPlace},
        dataflow::DataFlowAnalysis,
        lattice::FlatElem,
    },
    inst::{ImmIRegs, InstructionType, Reg, RegReg, SymRegs},
    ir::{Function, InstStore, RegType, Register},
};

pub fn remove_store_load(function: &mut Function, store: &mut InstStore) -> bool {
    // rewrite loads into just copies
    // if possible
    let mut const_analysis = ConstantMemoryAnalysis::new(function, store);
    let result = const_analysis.analyze(store);

    let mut change = false;

    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        for inst_index in 0..bb.len() {
            let id = bb[inst_index];
            let inst = store.get(id);
            match inst.data {
                InstructionType::Ld(Reg(addr)) => {
                    let state = &result[bb_index][inst_index];
                    match state.get(&MemoryPlace(addr)) {
                        Some(FlatElem::Value(val)) => {
                            change = true;
                            store.replace_inst(id, InstructionType::Mov(Reg(*val)), inst.reg_type);
                        }
                        Some(_) | None => (),
                    }
                }
                _ => (),
            }
        }
    }

    change |= remove_unused_stores(function, store);

    change |= remove_movs(function, store);

    change |= remove_unused_instruction(function, store);

    change
}

fn remove_unused_instruction(function: &mut Function, store: &InstStore) -> bool {
    let mut change = false;
    let used = function.get_used_regs(store);
    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        let mut inst_index = 0;
        while inst_index < bb.len() {
            if !used.contains(&bb[inst_index])
                && store.get(bb[inst_index]).reg_type != RegType::Void
            {
                bb.remove(inst_index);
                change = true;
            } else {
                inst_index += 1;
            }
        }
    }

    change
}

fn remove_unused_stores(function: &mut Function, store: &InstStore) -> bool {
    let mut change = false;
    let mut loads: Vec<Register> = vec![];
    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        for inst_index in 0..bb.len() {
            match &store.get(bb[inst_index]).data {
                InstructionType::Ld(Reg(addr)) => {
                    loads.push(*addr);
                }
                crate::inst::InstructionType::CallDirect(SymRegs(_, regs))
                | crate::inst::InstructionType::SysCall(ImmIRegs(_, regs)) => {
                    for reg in regs {
                        loads.push(*reg);
                    }
                }
                _ => (),
            }
        }
    }

    let mut pointer_analysis = AndersenAnalysis::new(function);
    let result = pointer_analysis.analyze(store);

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
            match store.get(bb[inst_index]).data {
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

fn remove_movs(function: &mut Function, store: &mut InstStore) -> bool {
    let mut change = false;
    let mut renames: HashMap<Register, Register> = HashMap::new();
    for bb_index in 0..function.blocks.len() {
        let bb = &mut function.blocks[bb_index];
        let mut inst_index = 0;
        while inst_index < bb.len() {
            match store.get(bb[inst_index]).data {
                InstructionType::Mov(Reg(reg)) => {
                    renames.insert(bb[inst_index], reg);
                    bb.remove(inst_index);
                    change = true;
                }
                _ => {
                    store.get_mut(bb[inst_index]).data.rename_regs(&renames);
                    inst_index += 1
                }
            }
        }
    }
    change
}
