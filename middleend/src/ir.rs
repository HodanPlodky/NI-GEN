use std::collections::HashMap;

use crate::inst::{BasicBlock, InstructionType, BBIndex, InstUUID};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    pub arg_count: u64,
    pub start: BasicBlock,
    pub blocks: Vec<BasicBlock>,
}

pub struct IrProgram {
    pub glob: BasicBlock,
    pub funcs: HashMap<String, Function>,
}

impl Default for IrProgram {
    fn default() -> Self {
        Self {
            glob: BasicBlock::new(vec![]),
            funcs: HashMap::new(),
        }
    }
}

pub struct IrBuilder {
    act_bb: BasicBlock,
    act_function: Option<Function>,
    prog: IrProgram,
}

impl Default for IrBuilder {
    fn default() -> Self {
        Self {
            act_bb: BasicBlock::default(),
            act_function: None,
            prog: IrProgram::default(),
        }
    }
}

impl IrBuilder {
    fn get_id(&self) -> InstUUID {
        todo!() 
    }

    pub fn add(&self, inst : InstructionType) {
        
    }

    pub fn add_astdata(&self, inst : InstructionType) {
        
    }
}
