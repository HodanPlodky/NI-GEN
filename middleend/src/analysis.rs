use std::collections::HashSet;

use crate::{
    inst::{BasicBlock, InstUUID, Register},
    ir::Function,
};

trait Lattice<A> {
    fn top(&self) -> A;
    fn bot(&self) -> A;
    fn lub(&self, a: &A, b: &A) -> A;
}

struct PowerSetLattice<E>
where
    E: std::hash::Hash + PartialEq + Eq + Clone + Copy,
{
    set: HashSet<E>,
}

impl<E> PowerSetLattice<E>
where
    E: std::hash::Hash + PartialEq + Eq + Clone + Copy,
{
    pub fn new(set: HashSet<E>) -> Self {
        Self { set }
    }
}

impl<E> Lattice<HashSet<E>> for PowerSetLattice<E>
where
    E: std::hash::Hash + PartialEq + Eq + Clone + Copy,
{
    fn top(&self) -> HashSet<E> {
        self.set.clone()
    }

    fn bot(&self) -> HashSet<E> {
        HashSet::new()
    }

    fn lub(&self, a: &HashSet<E>, b: &HashSet<E>) -> HashSet<E> {
        HashSet::union(a, b).copied().collect()
    }
}

/// Lattice that represents the state
/// of a function in the program
struct FunctionLattice<'a, A>
where
    A: Clone,
{
    function: &'a Function,
    inner_lattice: &'a dyn Lattice<A>,
}

impl<'a, A> FunctionLattice<'a, A>
where
    A: Clone,
{
    pub fn new(function: &'a Function, lattice: &'a dyn Lattice<A>) -> Self {
        Self {
            function,
            inner_lattice: lattice,
        }
    }
}

impl<A> Lattice<Vec<Vec<A>>> for FunctionLattice<'_, A>
where
    A: Clone,
{
    fn top(&self) -> Vec<Vec<A>> {
        todo!()
    }

    fn bot(&self) -> Vec<Vec<A>> {
        todo!()
    }

    fn lub(&self, a: &Vec<Vec<A>>, b: &Vec<Vec<A>>) -> Vec<Vec<A>> {
        todo!()
    }
}

enum DataflowType {
    Forwards,
    Backwards,
}

/// Represantation of the program lattice will be
/// just vector of vectors of lattice elements
/// for each instruction of the basic block
trait DataFlowAnalysis<'a, A, L>
where
    A: PartialEq + Clone + 'a,
    L: Lattice<A>,
{
    /// helper function for getting inner lattice
    fn inner_lattice(&self) -> &'a dyn Lattice<A>;
    /// helper function for getting function
    fn function(&self) -> &Function;
    /// helper function for type of analysis
    fn direction(&self) -> DataflowType;

    /// implementation of constrains
    fn transfer_fun(&mut self) -> A;

    fn before(&self, inst: InstUUID) -> Vec<InstUUID> {
        let (g, bb_index, insts_index) = inst;
        let func = self.function();
        match self.direction() {
            DataflowType::Forwards if func.blocks[bb_index].len() - 1 == insts_index => func.blocks
                [bb_index]
                .pred()
                .into_iter()
                .map(|x| (false, x, 0))
                .collect(),
            DataflowType::Forwards => {
                vec![(g, bb_index, insts_index + 1)]
            }
            DataflowType::Backwards if insts_index == 0 => func.blocks[bb_index]
                .succ()
                .into_iter()
                .map(|x| (false, x, func.blocks[x].len()))
                .collect(),
            DataflowType::Backwards => vec![(g, bb_index, insts_index - 1)],
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
        block.iter().map(|inst| self.join(inst.id, state)).collect()
    }

    /// function which aplies the tranfer function on
    /// every instuction in all basic blocks
    fn fun(&mut self, state: Vec<Vec<A>>) -> Vec<Vec<A>> {
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

struct LiveRegisterAnalysis {
    inner_lattice : PowerSetLattice<Register>,
}

impl<'a> DataFlowAnalysis<'a, HashSet<Register>, PowerSetLattice<Register>>
    for LiveRegisterAnalysis
{
    fn inner_lattice(&self) -> &'a dyn Lattice<HashSet<Register>> {
        todo!()
    }

    fn function(&self) -> &Function {
        todo!()
    }

    fn direction(&self) -> DataflowType {
        todo!()
    }

    fn transfer_fun(&mut self) -> HashSet<Register> {
        todo!()
    }
}
