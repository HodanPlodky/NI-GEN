use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use frontend::ast::AstData;

use crate::inst::{InstructionType, TerminatorBranch, TerminatorJump};

/// Id of the instruction
/// the bool flag signifies if the instruction
/// is part of the global space
pub type InstUUID = (bool, usize, usize);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Instruction {
    pub id: InstUUID,
    pub reg_type: RegType,
    pub ast_data: Option<AstData>,
    pub data: InstructionType,
}

impl Instruction {
    pub fn new(
        id: InstUUID,
        reg_type: RegType,
        ast_data: Option<AstData>,
        data: InstructionType,
    ) -> Self {
        Self {
            id,
            reg_type,
            ast_data,
            data,
        }
    }
}

impl From<Instruction> for Register {
    fn from(value: Instruction) -> Self {
        return value.id;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RegType {
    Void,
    Int,
    Char,
}

pub type Register = InstUUID;
pub type Symbol = String;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BasicBlock {
    predecesors: Vec<BBIndex>,
    pub instruction: Vec<Instruction>,
}

impl Default for BasicBlock {
    fn default() -> Self {
        Self {
            predecesors: vec![],
            instruction: vec![],
        }
    }
}

impl Deref for BasicBlock {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.instruction
    }
}

impl DerefMut for BasicBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instruction
    }
}

impl BasicBlock {
    pub fn add_predecesor(&mut self, predecesor: BBIndex) {
        self.predecesors.push(predecesor)
    }

    pub fn pred(&self) -> Vec<BBIndex> {
        self.predecesors.clone()
    }

    pub fn succ(&self) -> Vec<BBIndex> {
        use InstructionType::*;
        let inst = match self.last() {
            Some(inst) => inst,
            None => return vec![],
        };
        match &inst.data {
            Jmp(TerminatorJump(bbindex)) => vec![*bbindex],
            Branch(TerminatorBranch(_, bbindex_true, bbindex_false)) => {
                vec![*bbindex_true, *bbindex_false]
            }
            _ => [].to_vec(),
        }
    }

    pub fn get_used_regs(&self) -> Vec<Register> {
        self.iter().map(|x| x.data.get_regs()).flatten().collect()
    }

    pub fn terminated(&self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.last().unwrap().data.terminator()
        }
    }
}

pub type BBIndex = usize;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    pub name: String,
    pub arg_count: u64,
    pub ret_type: RegType,
    pub blocks: Vec<BasicBlock>,
}

impl Function {
    pub fn start(&self) -> &BasicBlock {
        &self.blocks[0]
    }

    pub fn get_used_regs(&self) -> Vec<Register> {
        self.iter().map(|x| x.get_used_regs()).flatten().collect()
    }
}

impl Default for Function {
    fn default() -> Self {
        Function {
            name: "global".to_string(),
            arg_count: 0,
            ret_type: RegType::Void,
            blocks: vec![],
        }
    }
}

impl Deref for Function {
    type Target = Vec<BasicBlock>;

    fn deref(&self) -> &Self::Target {
        &self.blocks
    }
}

impl DerefMut for Function {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.blocks
    }
}

#[derive(Debug)]
pub struct IrProgram {
    pub glob: Function,
    pub funcs: HashMap<String, Function>,
}

impl Default for IrProgram {
    fn default() -> Self {
        Self {
            glob: Function {
                name: "global".to_string(),
                arg_count: 0,
                ret_type: RegType::Void,
                blocks: vec![],
            },
            funcs: HashMap::new(),
        }
    }
}
