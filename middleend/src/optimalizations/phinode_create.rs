use crate::{
    analysis::{dataflow::DataFlowAnalysis, possible_mem::PossibleMemAnalysis},
    ir::Function,
};

pub fn create_phinodes(function: &mut Function) {
    let mut poss_analysis = PossibleMemAnalysis::new(function);
    let result = poss_analysis.analyze();
}
