use crate::ir::Function;

/// Position of the alloca
struct Cell(usize, usize);

pub struct AndersenAnalysis<'a> {
    function: &'a Function,
}

impl<'a> AndersenAnalysis<'a> {
    pub fn new(function: &'a Function) -> Self {
        Self { function }
    }

}


