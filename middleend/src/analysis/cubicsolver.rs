use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Deref, DerefMut},
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

    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.data.is_empty()
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
    data: MapToSet<T, (V, V)>,
}

impl<T, V> Default for Condition<T, V>
where
    T: Hash + Eq,
    V: Hash + Eq,
{
    fn default() -> Self {
        Self {
            data: MapToSet::default(),
        }
    }
}

impl<T, V> Deref for Condition<T, V>
where
    T: Hash + Eq,
    V: Hash + Eq,
{
    type Target = MapToSet<T, (V, V)>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, V> DerefMut for Condition<T, V>
where
    T: Hash + Eq,
    V: Hash + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
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

    pub fn solution(self) -> HashMap<V, HashSet<T>> {
        self.solution.data
    }

    pub fn add_token(&mut self, t: T, v: V) {
        if !self.solution.get_or_default(&v).contains(&t) {
            self.solution.get_or_default(&v).insert(t.clone());
            self.worklist.add(t, v)
        }
    }

    pub fn add_edge(&mut self, x: V, y: V) {
        if x != y && !self.succesor.get_or_default(&x).contains(&y) {
            self.succesor.get_or_default(&x).insert(y.clone());
            for t in self.solution.get_or_default(&x).clone().iter() {
                self.add_token(t.clone(), y.clone())
            }
        }
    }

    pub fn propagate(&mut self) {
        while let Some((t, x)) = self.worklist.next() {
            if !self.conditions.contains_key(&x) {
                self.conditions.insert(x.clone(), Condition::default());
            }
            for (y, z) in self
                .conditions
                .get_mut(&x)
                .unwrap()
                .get_or_default(&t)
                .clone()
                .iter()
            {
                self.add_edge(y.clone(), z.clone());
            }

            for y in self.succesor.get_or_default(&x).clone().iter() {
                self.add_token(t.clone(), y.clone());
            }
        }
    }

    pub fn includes(&mut self, t: T, v: V) {
        self.add_token(t, v);
        self.propagate();
    }

    pub fn includes_implies(&mut self, t: T, x: V, y: V, z: V) {
        if self.solution.get_or_default(&x).contains(&t) {
            self.add_edge(y, z);
            self.propagate();
        } else {
            if !self.conditions.contains_key(&x) {
                self.conditions.insert(x.clone(), Condition::default());
            }
            self.conditions
                .get_mut(&x)
                .unwrap()
                .get_or_default(&t)
                .insert((y, z));
        }
    }
}
