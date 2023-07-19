use std::collections::HashMap;

use crate::ir::IrProgram;

use super::{lattice::Lattice, dataflow::DataFlowAnalysis};

pub fn analyze_program<'a, A, L>(
    program: &'a IrProgram,
    dataflowanalysis: impl DataFlowAnalysis<'a, A, L>,
) -> HashMap<String, Vec<Vec<A>>>
where
    A: PartialEq + Clone + 'a,
    L: Lattice<A>,
{
    let mut dataflowanalysis = dataflowanalysis;
    program
        .funcs
        .iter()
        .map(|(name, func)| {
            dataflowanalysis.set_function(func);
            (name.clone(), dataflowanalysis.analyze())
        })
        .collect()
}
