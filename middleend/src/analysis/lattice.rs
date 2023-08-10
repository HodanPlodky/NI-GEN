use std::collections::{HashMap, HashSet};

use crate::ir::Function;

pub trait Lattice<A> {
    fn top(&self) -> A;
    fn bot(&self) -> A;
    fn lub(&self, a: &A, b: &A) -> A;
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

pub struct MapLattce<'a, F, T>
where
    F: std::hash::Hash + PartialEq + Eq + Clone + Copy,
    T: std::hash::Hash + PartialEq + Eq + Clone + Copy,
{
    map: HashSet<F>,
    inner_lattice: &'a dyn Lattice<T>,
}

impl<'a, F, T> MapLattce<'a, F, T>
where
    F: std::hash::Hash + PartialEq + Eq + Clone + Copy,
    T: std::hash::Hash + PartialEq + Eq + Clone + Copy,
{
    pub fn new(map: HashSet<F>, inner_lattice: &'a dyn Lattice<T>) -> Self {
        Self { map, inner_lattice }
    }
}

impl<'a, F, T> Lattice<HashMap<F, T>> for MapLattce<'a, F, T>
where
    F: std::hash::Hash + PartialEq + Eq + Clone + Copy,
    T: std::hash::Hash + PartialEq + Eq + Clone + Copy,
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
