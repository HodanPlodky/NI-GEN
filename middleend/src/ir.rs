use std::collections::HashMap;

use crate::inst::{BBIndex, BasicBlock, InstUUID, Instruction, InstructionType, RegType};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    pub arg_count: u64,
    pub start: BasicBlock,
    pub blocks: Vec<BasicBlock>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct IrBuilder {
    global: bool,
    act_bb: BasicBlock,
    act_function: Option<Function>,
    prog: IrProgram,
}

impl Default for IrBuilder {
    fn default() -> Self {
        Self {
            global: true,
            act_bb: BasicBlock::default(),
            act_function: None,
            prog: IrProgram::default(),
        }
    }
}

#[derive(Debug)]
pub enum IrBuilderError {
    BasicBlockNotTerminated,
    NotGlobalBB,
    GlobalBBInFn,
    NotInFunction,
    NotFunction,
}

impl IrBuilder {
    fn get_id(&self) -> InstUUID {
        if self.global {
            (true, 0, self.act_bb.len())
        } else if let Some(func) = &self.act_function {
            (false, func.blocks.len() + 1, self.act_bb.len())
        } else {
            (false, 0, self.act_bb.len())
        }
    }

    pub fn add(&mut self, inst: InstructionType, reg_type: RegType) {
        let inst = Instruction::new(self.get_id(), reg_type, None, inst);
        self.act_bb.push(inst);
    }

    pub fn create_bb(&mut self) {
        self.act_bb = BasicBlock::default();
    }

    pub fn set_global(&mut self) -> Result<(), IrBuilderError> {
        if !self.global {
            Err(IrBuilderError::NotGlobalBB)
        } else if self.act_bb.terminated() {
            self.global = false;
            self.prog.glob = std::mem::replace(&mut self.act_bb, BasicBlock::default());
            Ok(())
        } else {
            Err(IrBuilderError::BasicBlockNotTerminated)
        }
    }

    pub fn create_fn(&mut self, arg_count: u64) -> Result<(), IrBuilderError> {
        if self.global {
            return Err(IrBuilderError::GlobalBBInFn);
        }
        self.act_function = Some(Function {
            start: std::mem::replace(&mut self.act_bb, BasicBlock::default()),
            arg_count,
            blocks: vec![],
        });
        Ok(())
    }

    pub fn store_fn(&mut self, name: &str) -> Result<(), IrBuilderError> {
        let tmp = std::mem::replace(&mut self.act_function, None);
        if let Some(func) = tmp {
            self.prog.funcs.insert(name.to_string(), func);
            Ok(())
        } else {
            Err(IrBuilderError::NotFunction)
        }
    }

    pub fn append_bb(&mut self) -> Result<(), IrBuilderError> {
        if let Some(func) = &mut self.act_function {
            func.blocks
                .push(std::mem::replace(&mut self.act_bb, BasicBlock::default()));
            Ok(())
        } else {
            Err(IrBuilderError::NotInFunction)
        }
    }

    pub fn add_astdata(&self, _inst: InstructionType) {
        // this is just for better comp so fuck it
        todo!()
    }
}
