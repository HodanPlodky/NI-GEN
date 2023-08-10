use crate::ir::{Function, Register};

struct MemoryPlace(Register);

pub struct ConstantMemoryAnalysis<'a> {
    function : &'a Function,
    //inner_lattice : MapLattice<MemoryPlace, >
}
