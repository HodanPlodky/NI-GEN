use crate::ir::{Function, Instruction};

fn remove_store_load(function: &mut Function) {
    // removed unused registers
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
