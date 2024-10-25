use std::collections::{HashMap, HashSet};

use crate::{
    analysis::lattice::Lattice,
    inst::{ImmIRegs, InstructionType, RegReg, SymRegs},
    ir::{Function, Register},
};

use super::{
    dataflow::{DataFlowAnalysis, DataflowType, InstPos},
    lattice::{FlatElem, FlatLattice, MapLattice},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryPlace(pub Register);

type ConstLattice = MapLattice<FlatLattice<Register>, MemoryPlace, FlatElem<Register>>;

pub struct ConstantMemoryAnalysis<'a> {
    function: &'a Function,
    inner_lattice: ConstLattice,
}

impl<'a> ConstantMemoryAnalysis<'a> {
    pub fn new(function: &'a Function) -> Self {
        Self {
            function,
            inner_lattice: MapLattice::new(
                ConstantMemoryAnalysis::get_stores(function),
                FlatLattice::new(),
            ),
        }
    }

    fn get_stores(function: &'a Function) -> HashSet<MemoryPlace> {
        function
            .blocks
            .iter()
            .map(|x| {
                x.iter()
                    .filter(|x| match x.data {
                        InstructionType::St(_) => true,
                        _ => false,
                    })
                    .map(|x| match x.data {
                        InstructionType::St(RegReg(addr, _)) => MemoryPlace(addr),
                        _ => unreachable!(),
                    })
                    .collect::<Vec<MemoryPlace>>()
            })
            .flatten()
            .collect()
    }
}

impl<'a> DataFlowAnalysis<'a, HashMap<MemoryPlace, FlatElem<Register>>, ConstLattice>
    for ConstantMemoryAnalysis<'a>
{
    fn inner_lattice(
        &self,
    ) -> &dyn super::lattice::Lattice<HashMap<MemoryPlace, FlatElem<Register>>> {
        &self.inner_lattice
    }

    fn function(&self) -> &Function {
        self.function
    }

    fn set_function(&mut self, func: &'a Function) {
        self.function = func;
        self.inner_lattice =
            MapLattice::new(ConstantMemoryAnalysis::get_stores(func), FlatLattice::new());
    }

    fn direction(&self) -> super::dataflow::DataflowType {
        DataflowType::Forwards
    }

    fn transfer_fun(
        &self,
        inst: &crate::ir::Instruction,
        pos: InstPos,
        state: HashMap<MemoryPlace, FlatElem<Register>>,
    ) -> HashMap<MemoryPlace, FlatElem<Register>> {
        use InstructionType::*;
        let (_, bb_idx, inst_idx) = pos;
        match &inst.data {
            St(RegReg(addr, reg)) => {
                let mut state = state;
                state.insert(MemoryPlace(*addr), FlatElem::Value(*reg));
                state
            }
            CallDirect(SymRegs(_, regs)) | SysCall(ImmIRegs(_, regs)) => {
                let mut state = state;
                for reg in regs.iter() {
                    if self.inner_lattice.map.contains(&MemoryPlace(*reg)) {
                        state.insert(MemoryPlace(*reg), FlatElem::Top);
                    }
                }
                state
            }
            _ if bb_idx == 0 && inst_idx == 0 => self.inner_lattice.bot(),
            _ => state,
        }
    }
}
