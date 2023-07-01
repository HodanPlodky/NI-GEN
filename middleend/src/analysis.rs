use std::collections::HashSet;

use crate::{
    inst::{BasicBlock, InstUUID, InstructionType, Register},
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
        self.function
            .blocks
            .iter()
            .map(|x| x.iter().map(|_| self.inner_lattice.bot()).collect())
            .collect()
    }

    fn bot(&self) -> Vec<Vec<A>> {
        self.function
            .blocks
            .iter()
            .map(|x| x.iter().map(|_| self.inner_lattice.top()).collect())
            .collect()
    }

    fn lub(&self, a: &Vec<Vec<A>>, b: &Vec<Vec<A>>) -> Vec<Vec<A>> {
        a.iter()
            .zip(b.iter())
            .map(|(a, b)| {
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| self.inner_lattice.lub(a, b))
                    .collect()
            })
            .collect()
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
    fn inner_lattice(&self) -> &dyn Lattice<A>;
    /// helper function for getting function
    fn function(&self) -> &Function;
    /// helper function for type of analysis
    fn direction(&self) -> DataflowType;

    /// implementation of constrains
    fn transfer_fun(&self, inst: InstUUID, state: Vec<Vec<A>>) -> A;

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
    fn fun(&self, state: Vec<Vec<A>>) -> Vec<Vec<A>> {
        let func = self.function();
        func.blocks
            .iter()
            .map(|block| self.fun_block(&state, block))
            .collect()
    }

    /// basic algorighm for finding fixed point
    fn analyze(&'a mut self) -> Vec<Vec<A>> {
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

struct LiveRegisterAnalysis<'a> {
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
        inst: InstUUID,
        state: Vec<Vec<HashSet<Register>>>,
    ) -> HashSet<Register> {
        use InstructionType::*;

        let blocks = self.function();
        let (_, bb_index, inst_index) = inst;
        let inst = blocks[bb_index][inst_index].clone();
        match inst.data {
            Ret(_) | Retr(_) => self.inner_lattice.bot(),
            _ => state[bb_index][inst_index].clone(),
        }
    }
}
