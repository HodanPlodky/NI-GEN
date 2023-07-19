use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

struct WorkList<T, V> {
    data: Vec<(T, V)>,
}

impl<T, V> Default for WorkList<T, V> {
    fn default() -> Self {
        Self { data: vec![] }
    }
}

impl<T, V> WorkList<T, V> {
    fn add(&mut self, t: T, v: V) {
        self.data.push((t, v));
    }

    fn empty(&self) -> bool {
        self.empty()
    }

    fn next(&mut self) -> Option<(T, V)> {
        self.data.pop()
    }
}

struct Condition<T, V>
where
    T: Hash + Eq,
    V: Hash + Eq,
{
    data: HashMap<T, HashSet<(V, V)>>,
}

struct MapToSet<F, T>
where
    F: Hash + Eq,
    T: Hash + Eq,
{
    data: HashMap<F, HashSet<T>>,
}

impl<F, T> Default for MapToSet<F, T>
where
    F: Hash + Eq,
    T: Hash + Eq,
{
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}

impl<F, T> MapToSet<F, T>
where
    F: Hash + Eq + Clone,
    T: Hash + Eq,
{
    fn get_or_default(&mut self, v: &F) -> &mut HashSet<T> {
        if !self.data.contains_key(v) {
            self.data.insert(v.clone(), HashSet::new());
        }
        self.data.get_mut(v).unwrap()
    }
}

pub struct CubicSolver<T, V>
where
    T: Hash + Eq + Clone,
    V: Hash + Eq + Clone,
{
    worklist: WorkList<T, V>,
    solution: MapToSet<V, T>,
    succesor: MapToSet<V, V>,
    conditions: HashMap<V, Condition<T, V>>,
}

impl<T, V> CubicSolver<T, V>
where
    T: Hash + Eq + Clone,
    V: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            worklist: WorkList::default(),
            solution: MapToSet::default(),
            succesor: MapToSet::default(),
            conditions: HashMap::default(),
        }
    }

    pub fn solution(&self) -> &HashMap<V, HashSet<T>> {
        &self.solution.data
    }

    pub fn addToken(&mut self, t: T, v: V) {
        if !self.solution.get_or_default(&v).contains(&t) {
            self.solution.get_or_default(&v).insert(t.clone());
            self.worklist.add(t, v)
        }
    }

    pub fn addEdge(&mut self, x: V, y: V) {
        if x != y && !self.succesor.get_or_default(&x).contains(&y) {
            self.succesor.get_or_default(&x).insert(y.clone());
            for t in self.solution.get_or_default(&x).clone().iter() {
                self.addToken(t.clone(), y.clone())
            }
        }
    }

    pub fn propagate(&mut self) {
        while let Some((t, x)) = self.worklist.next() {
            todo!()
        }
    }

    pub fn includes(&mut self, t: T, v: V) {
        todo!()
    }

    pub fn includesImplies(&mut self, T: T, x: V, y: V, z: V) {
        todo!()
    }
}
