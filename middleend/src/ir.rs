use std::collections::HashMap;

use crate::inst::BasicBlock;

pub struct Function {
    arg_count: u64,
    start: BasicBlock,
    blocks: Vec<BasicBlock>,
}

pub struct IrProgram {
    glob: BasicBlock,
    funcs: HashMap<String, Function>,
}
