use crate::ir::{BasicBlock, Function, InstUUID};

use super::lattice::{FunctionLattice, Lattice};

pub enum DataflowType {
    Forwards,
    Backwards,
}

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
    fn set_function(&mut self, func : &'a Function);
    /// helper function for type of analysis
    fn direction(&self) -> DataflowType;

    /// implementation of constrains
    fn transfer_fun(&self, inst: InstUUID, state: A) -> A;

    fn before(&self, inst: InstUUID) -> Vec<InstUUID> {
        let (g, bb_index, insts_index) = inst;
        let func = self.function();
        match self.direction() {
            DataflowType::Backwards if func.blocks[bb_index].len() - 1 == insts_index => func
                .blocks[bb_index]
                .succ()
                .into_iter()
                .map(|x| (false, x, 0))
                .collect(),
            DataflowType::Backwards => {
                vec![(g, bb_index, insts_index + 1)]
            }
            DataflowType::Forwards if insts_index == 0 => func.blocks[bb_index]
                .pred()
                .into_iter()
                .map(|x| (false, x, func.blocks[x].len() - 1))
                .collect(),
            DataflowType::Forwards => vec![(g, bb_index, insts_index - 1)],
        }
    }

    fn join(&self, inst: InstUUID, state: &Vec<Vec<A>>) -> A {
        let prev = self
            .before(inst)
            .into_iter()
            .map(|(_, bb, inst)| state[bb][inst].clone());
        prev.fold(self.inner_lattice().bot(), |acc, x| {
            self.inner_lattice().lub(&acc, &x)
        })
    }

    fn fun_block(&self, state: &Vec<Vec<A>>, block: &BasicBlock) -> Vec<A> {
        block
            .iter()
            .map(|inst| self.transfer_fun(inst.id, self.join(inst.id, state)))
            .collect()
    }

    /// function which aplies the tranfer function on
    /// every instuction in all basic blocks
    fn fun(&self, state: Vec<Vec<A>>) -> Vec<Vec<A>> {
        let func = self.function();
        func.blocks
            .iter()
            .map(|block| self.fun_block(&state, block))
            .collect()
    }

    /// basic algorighm for finding fixed point
    fn analyze(&mut self) -> Vec<Vec<A>> {
        let fun_lattice = FunctionLattice::<A>::new(self.function(), self.inner_lattice());
        let mut x = fun_lattice.bot();
        loop {
            let t = x.clone();
            x = self.fun(x);

            if t == x {
                break;
            }
        }
        x
    }
}
