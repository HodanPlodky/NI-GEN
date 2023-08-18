use std::collections::HashSet;

use crate::ir::{Register, Function};

use super::{
    const_mem::MemoryPlace,
    lattice::{MapLattice, PowerSetLattice},
};

type PossibleLattice = MapLattice<PowerSetLattice<Register>, MemoryPlace, HashSet<Register>>;

struct PossibleMemAnalysis<'a> {
    function : &'a Function,
    //inner_lattice : PossibleLattice,
}
