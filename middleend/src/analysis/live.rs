use std::collections::HashSet;

use crate::{
    inst::InstructionType,
    ir::{Function, InstUUID, Instruction, Register},
};

use super::{
    dataflow::{DataFlowAnalysis, DataflowType, InstPos},
    lattice::{Lattice, PowerSetLattice},
};

pub struct LiveRegisterAnalysis<'a> {
    function: &'a Function,
    inner_lattice: PowerSetLattice<Register>,
}

impl<'a> LiveRegisterAnalysis<'a> {
    pub fn new(function: &'a Function) -> Self {
        let registers: HashSet<Register> = HashSet::from_iter(
            function
                .blocks
                .iter()
                .map(|x| x.iter().map(|y| y.id))
                .flatten(),
        );
        let inner_lattice = PowerSetLattice::new(registers);

        Self {
            function,
            inner_lattice,
        }
    }
}

impl<'a> DataFlowAnalysis<'a, HashSet<Register>, PowerSetLattice<Register>>
    for LiveRegisterAnalysis<'a>
{
    fn inner_lattice(&self) -> &dyn Lattice<HashSet<Register>> {
        &self.inner_lattice
    }

    fn function(&self) -> &Function {
        self.function
    }

    fn direction(&self) -> DataflowType {
        DataflowType::Backwards
    }

    fn transfer_fun(
        &self,
        inst: &Instruction,
        _pos: InstPos,
        state: HashSet<Register>,
    ) -> HashSet<Register> {
        use InstructionType::*;

        match inst.data {
            Ret(_) | Exit(_) => self.inner_lattice.bot(),
            Retr(_) => HashSet::from_iter(inst.data.get_regs().into_iter()),
            _ => {
                let mut state = state;
                state.remove(&inst.id);
                for reg in inst.data.get_regs() {
                    state.insert(reg);
                }
                state
            }
        }
    }

    fn set_function(&mut self, function: &'a Function) {
        self.function = function;
    }
}
