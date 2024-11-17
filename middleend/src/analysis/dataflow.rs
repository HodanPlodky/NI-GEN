use crate::ir::{Function, InstStore, Instruction};

use super::lattice::{FunctionLattice, Lattice};

pub enum DataflowType {
    Forwards,
    Backwards,
}

pub type InstPos = (bool, usize, usize);

/// Represantation of the program lattice will be
/// just vector of vectors of lattice elements
/// for each instruction of the basic block
pub trait DataFlowAnalysis<'a, A, L>
where
    A: PartialEq + Clone + 'a,
    L: Lattice<A>,
{
    /// helper function for getting inner lattice
    fn inner_lattice(&self) -> &dyn Lattice<A>;
    /// helper function for getting function
    fn function(&self) -> &Function;
    /// helper function for setting function
    fn set_function(&mut self, func: &'a Function);
    /// helper function for type of analysis
    fn direction(&self) -> DataflowType;

    /// implementation of constrains
    fn transfer_fun(&self, inst: &Instruction, pos: InstPos, state: A) -> A;

    // returns possitions
    fn before(&self, inst: InstPos, store: &InstStore) -> Vec<InstPos> {
        let (g, bb_index, inst_index) = inst;
        let func = self.function();
        match self.direction() {
            DataflowType::Backwards if func.blocks[bb_index].len() - 1 == inst_index => func.blocks
                [bb_index]
                .succ(store)
                .into_iter()
                .map(|x| (false, x, 0))
                .collect(),
            DataflowType::Backwards => {
                vec![(g, bb_index, inst_index + 1)]
            }
            DataflowType::Forwards if inst_index == 0 => func.blocks[bb_index]
                .pred()
                .into_iter()
                .map(|x| (false, x, func.blocks[x].len() - 1))
                .collect(),
            DataflowType::Forwards => vec![(g, bb_index, inst_index - 1)],
        }
    }

    fn join(&self, inst: InstPos, state: &Vec<Vec<A>>, store: &InstStore) -> A {
        let prev = self
            .before(inst, store)
            .into_iter()
            .map(|(_, bb, inst)| state[bb][inst].clone());
        prev.fold(self.inner_lattice().bot(), |acc, x| {
            self.inner_lattice().lub(&acc, &x)
        })
    }

    fn fun_block(
        &self,
        state: &Vec<Vec<A>>,
        block_idx: usize,
        global: bool,
        store: &InstStore,
    ) -> Vec<A> {
        (0..self.function()[block_idx].len())
            .map(|x| (global, block_idx, x))
            .map(|pos| {
                self.transfer_fun(
                    store.get(self.function()[block_idx][pos.2]),
                    pos,
                    self.join(pos, state, store),
                )
            })
            .collect()
    }

    /// function which aplies the tranfer function on
    /// every instuction in all basic blocks
    fn fun(&self, state: Vec<Vec<A>>, store: &InstStore) -> Vec<Vec<A>> {
        let func = self.function();
        func.blocks
            .iter()
            .enumerate()
            .map(|(idx, _)| self.fun_block(&state, idx, false, store))
            .collect()
    }

    /// basic algorighm for finding fixed point
    fn analyze(&mut self, store: &InstStore) -> Vec<Vec<A>> {
        let fun_lattice = FunctionLattice::<A>::new(self.function(), self.inner_lattice());
        let mut x = fun_lattice.bot();
        loop {
            let t = x.clone();
            x = self.fun(x, store);

            if t == x {
                break;
            }
        }
        x
    }
}
