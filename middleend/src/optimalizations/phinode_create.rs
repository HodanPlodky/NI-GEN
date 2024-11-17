use crate::{
    analysis::{dataflow::DataFlowAnalysis, possible_mem::PossibleMemAnalysis},
    ir::{Function, InstStore},
};

pub fn create_phinodes(function: &mut Function, store: &InstStore) {
    let mut poss_analysis = PossibleMemAnalysis::new(function, store);
    let result = poss_analysis.analyze(store);
}
