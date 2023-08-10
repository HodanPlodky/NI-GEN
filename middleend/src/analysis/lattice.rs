use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use crate::ir::Function;

pub trait Lattice<A> {
    fn top(&self) -> A;
    fn bot(&self) -> A;
    fn lub(&self, a: &A, b: &A) -> A;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlatElem<T>
where
    T: std::hash::Hash + PartialEq + Eq + Clone + Copy,
{
    Top,
    Value(T),
    Bot,
}

pub struct FlatLattice<T>
where
    T: PartialEq + Eq + Clone + Copy,
{
    phantom: PhantomData<T>,
}

impl<T> FlatLattice<T>
where
    T: std::hash::Hash + PartialEq + Eq + Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T> Lattice<FlatElem<T>> for FlatLattice<T>
where
    T: std::hash::Hash + PartialEq + Eq + Clone + Copy,
{
    fn top(&self) -> FlatElem<T> {
        FlatElem::Top
    }

    fn bot(&self) -> FlatElem<T> {
        FlatElem::Bot
    }

    fn lub(&self, a: &FlatElem<T>, b: &FlatElem<T>) -> FlatElem<T> {
        use FlatElem::*;
        match (a, b) {
            (Top, _) => Top,
            (_, Top) => Top,
            (Value(x), Value(y)) if x == y => Value(*x),
            (Value(x), Value(y)) => Top,
            (x, Bot) | (Bot, x) => *x,
        }
    }
}

pub struct PowerSetLattice<E>
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

pub struct MapLattice<L, F, T>
where
    F: std::hash::Hash + PartialEq + Eq + Clone + Copy,
    T: std::hash::Hash + PartialEq + Eq + Clone + Copy,
    L: Lattice<T>,
{
    map: HashSet<F>,
    inner_lattice: L,
    phantom: PhantomData<T>,
}

impl<L, F, T> MapLattice<L, F, T>
where
    F: std::hash::Hash + PartialEq + Eq + Clone + Copy,
    T: std::hash::Hash + PartialEq + Eq + Clone + Copy,
    L: Lattice<T>,
{
    pub fn new(map: HashSet<F>, inner_lattice: L) -> Self {
        Self {
            map,
            inner_lattice,
            phantom: PhantomData::default(),
        }
    }
}

impl<L, F, T> Lattice<HashMap<F, T>> for MapLattice<L, F, T>
where
    F: std::hash::Hash + PartialEq + Eq + Clone + Copy,
    T: std::hash::Hash + PartialEq + Eq + Clone + Copy,
    L: Lattice<T>,
{
    fn top(&self) -> HashMap<F, T> {
        HashMap::from_iter(
            self.map
                .iter()
                .map(|x| (x.clone(), self.inner_lattice.top())),
        )
    }

    fn bot(&self) -> HashMap<F, T> {
        HashMap::from_iter(
            self.map
                .iter()
                .map(|x| (x.clone(), self.inner_lattice.bot())),
        )
    }

    fn lub(&self, a: &HashMap<F, T>, b: &HashMap<F, T>) -> HashMap<F, T> {
        HashMap::from_iter(
            self.map
                .iter()
                .map(|x| (x.clone(), self.inner_lattice.lub(&a[x], &b[x]))),
        )
    }
}

/// Lattice that represents the state
/// of a function in the program
pub struct FunctionLattice<'a, A>
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
            .map(|x| x.iter().map(|_| self.inner_lattice.top()).collect())
            .collect()
    }

    fn bot(&self) -> Vec<Vec<A>> {
        self.function
            .blocks
            .iter()
            .map(|x| x.iter().map(|_| self.inner_lattice.bot()).collect())
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
