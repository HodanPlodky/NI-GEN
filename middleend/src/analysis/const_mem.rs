use std::collections::{HashSet, HashMap};

use crate::ir::{Function, Register};

use super::{lattice::{FlatElem, FlatLattice, MapLattice}, dataflow::{DataFlowAnalysis, DataflowType}};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct MemoryPlace(Register);

type ConstLattice = MapLattice<FlatLattice<Register>, MemoryPlace, FlatElem<Register>>;

pub struct ConstantMemoryAnalysis<'a> {
    function: &'a Function,
    inner_lattice: ConstLattice,
}

impl<'a> ConstantMemoryAnalysis<'a> {
    pub fn new(
        function: &'a Function,
    ) -> Self {
        Self {
            function,
            inner_lattice : MapLattice::new(HashSet::new(), FlatLattice::new()),
        }
    }
}

impl<'a> DataFlowAnalysis<'a, HashMap<MemoryPlace, FlatElem<Register>>, ConstLattice> for ConstantMemoryAnalysis<'a> {
    fn inner_lattice(&self) -> &dyn super::lattice::Lattice<HashMap<MemoryPlace, FlatElem<Register>>> {
        &self.inner_lattice
    }

    fn function(&self) -> &Function {
        self.function
    }

    fn set_function(&mut self, func : &'a Function) {
        self.function = func;
    }

    fn direction(&self) -> super::dataflow::DataflowType {
        DataflowType::Forwards
    }

    fn transfer_fun(&self, inst: crate::ir::InstUUID, state: HashMap<MemoryPlace, FlatElem<Register>>) -> HashMap<MemoryPlace, FlatElem<Register>> {
        todo!()
    }
}
