use std::collections::{HashMap, HashSet};

use crate::{
    analysis::lattice::Lattice,
    inst::{InstructionType, RegReg, SymRegs},
    ir::{Function, InstStore, Instruction, Register},
};

use super::{
    const_mem::MemoryPlace,
    dataflow::{DataFlowAnalysis, DataflowType, InstPos},
    lattice::{MapLattice, PowerSetLattice},
};

type PossibleLattice = MapLattice<PowerSetLattice<Register>, MemoryPlace, HashSet<Register>>;

pub struct PossibleMemAnalysis<'a> {
    function: &'a Function,
    store: &'a InstStore,
    inner_lattice: PossibleLattice,
}

impl<'a> PossibleMemAnalysis<'a> {
    pub fn new(function: &'a Function, store: &'a InstStore) -> Self {
        let regs = HashSet::from_iter(function.get_used_regs(store).into_iter());
        Self {
            function,
            store,
            inner_lattice: MapLattice::new(
                PossibleMemAnalysis::get_stores(function, store),
                PowerSetLattice::new(regs),
            ),
        }
    }

    fn get_stores(function: &'a Function, store: &InstStore) -> HashSet<MemoryPlace> {
        function
            .blocks
            .iter()
            .map(|x| {
                x.iter()
                    .filter(|x| match store.get(**x).data {
                        InstructionType::St(_) => true,
                        _ => false,
                    })
                    .map(|x| match store.get(*x).data {
                        InstructionType::St(RegReg(addr, _)) => MemoryPlace(addr),
                        _ => unreachable!(),
                    })
                    .collect::<Vec<MemoryPlace>>()
            })
            .flatten()
            .collect()
    }
}

impl<'a> DataFlowAnalysis<'a, HashMap<MemoryPlace, HashSet<Register>>, PossibleLattice>
    for PossibleMemAnalysis<'a>
{
    fn inner_lattice(
        &self,
    ) -> &dyn super::lattice::Lattice<HashMap<MemoryPlace, HashSet<Register>>> {
        &self.inner_lattice
    }

    fn function(&self) -> &Function {
        self.function
    }

    fn set_function(&mut self, func: &'a Function) {
        self.function = func;
        self.inner_lattice = MapLattice::new(
            PossibleMemAnalysis::get_stores(func, self.store),
            PowerSetLattice::new(func.get_used_regs(self.store).into_iter().collect()),
        );
    }

    fn direction(&self) -> super::dataflow::DataflowType {
        DataflowType::Forwards
    }

    fn transfer_fun(
        &self,
        inst: &Instruction,
        pos: InstPos,
        state: HashMap<MemoryPlace, HashSet<Register>>,
    ) -> HashMap<MemoryPlace, HashSet<Register>> {
        use InstructionType::*;
        let (_, bb_index, inst_index) = pos;
        match &inst.data {
            St(RegReg(addr, reg)) => {
                let mut state = state;
                state.get_mut(&MemoryPlace(*addr)).unwrap().insert(*reg);
                state
            }
            CallDirect(SymRegs(_, regs)) => {
                let mut state = state;
                for reg in regs.iter() {
                    if self.inner_lattice.map.contains(&MemoryPlace(*reg)) {
                        state.insert(MemoryPlace(*reg), self.inner_lattice.inner_lattice.top());
                    }
                }
                state
            }
            _ if bb_index == 0 && inst_index == 0 => self.inner_lattice.bot(),
            _ => state,
        }
    }
}
