use std::collections::{HashMap, HashSet};

struct WorkList<T, V> {
    data: Vec<(T, V)>,
}

impl<T, V> Default for WorkList<T, V> {
    fn default() -> Self {
        Self { data: vec![] }
    }
}

struct Condition<T, V> {
    data: HashMap<T, HashSet<(V, V)>>,
}

struct CubicSolver<T, V> {
    tokens: HashSet<T>,
    variables: HashSet<V>,
    worklist: WorkList<T, V>,
    solution: HashMap<V, HashSet<T>>,
    succesor: HashMap<V, HashSet<T>>,
    conditions: HashMap<V, Condition<T, V>>,
}

impl<T, V> CubicSolver<T, V> {
    pub fn new(tokens: HashSet<T>, variables: HashSet<V>) -> Self {
        Self {
            tokens,
            variables,
            worklist: WorkList::default(),
            solution: HashMap::default(),
            succesor: HashMap::default(),
            conditions: HashMap::default(),
        }
    }
}
