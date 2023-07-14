use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use frontend::ast::AstData;

use crate::inst::{InstructionType, TerminatorJump};

// ----------------------------------------------------------------------
// Data structures
// ----------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct BBIndex(pub usize);

impl BBIndex {
    pub fn index(&self) -> usize {
        self.0
    }

    pub fn first(&self) -> bool {
        self.0 == 0
    }
}

impl Display for BBIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index())
    }
}

/// Id of the instruction

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct InstUUID(pub usize);

/// the bool flag signifies if the instruction
/// is part of the global space
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct InstIndex(pub bool, pub BBIndex, pub usize);

pub type Register = InstUUID;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Symbol(pub String);

impl Into<String> for Symbol {
    fn into(self) -> String {
        self.0
    }
}

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

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RegType {
    Void,
    Int,
    Char,
}

#[derive(Clone, Debug)]
pub struct InstructionStore {
    instructions: Vec<Instruction>,
}

impl Default for InstructionStore {
    fn default() -> Self {
        Self {
            instructions: vec![],
        }
    }
}

impl InstructionStore {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    pub fn get_inst(&self, inst: InstUUID) -> Option<&Instruction> {
        let InstUUID(index) = inst;
        self.instructions.get(index)
    }

    pub fn create_id(&self) -> InstUUID {
        InstUUID(self.instructions.len())
    }

    pub fn add_instruction(&mut self, inst: Instruction) -> InstUUID {
        self.instructions.push(inst);
        InstUUID(self.instructions.len() - 1)
    }
}

#[derive(Clone, Debug)]
pub struct BasicBlock<'a> {
    store: &'a InstructionStore,
    predecesors: Vec<BBIndex>,
    pub instruction: Vec<InstUUID>,
}

impl<'a> Deref for BasicBlock<'a> {
    type Target = Vec<&'a Instruction>;

    fn deref(&self) -> &Self::Target {
        &self
            .instruction
            .iter()
            .map(|x| self.store.get_inst(*x).unwrap())
            .collect()
    }
}

impl PartialEq for BasicBlock<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.predecesors == other.predecesors && self.instruction == other.instruction
    }
}

impl Eq for BasicBlock<'_> {}

impl<'a> BasicBlock<'a> {
    pub fn new(store: &'a InstructionStore) -> Self {
        Self {
            store,
            predecesors: vec![],
            instruction: vec![],
        }
    }

    pub fn add_predecesor(&mut self, predecesor: BBIndex) {
        self.predecesors.push(predecesor)
    }

    pub fn push(&mut self, inst : InstUUID) {
        self.instruction.push(inst)
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
            Branch(crate::inst::TerminatorBranch(_, bbindex_true, bbindex_false)) => {
                vec![*bbindex_true, *bbindex_false]
            }
            _ => [].to_vec(),
        }
    }

    pub fn terminated(&self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.last().unwrap().data.terminator()
        }
    }
}

impl From<Instruction> for Register {
    fn from(value: Instruction) -> Self {
        return value.id;
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function<'a> {
    pub name: String,
    pub arg_count: u64,
    pub ret_type: RegType,
    pub blocks: Vec<BasicBlock<'a>>,
}

impl Function<'_> {
    pub fn start(&self) -> &BasicBlock {
        &self.blocks[0]
    }

    pub fn get_bb(&self, bb_index: BBIndex) -> &BasicBlock {
        &self.blocks[bb_index.index()]
    }
}

impl Default for Function<'_> {
    fn default() -> Self {
        Function {
            name: "global".to_string(),
            arg_count: 0,
            ret_type: RegType::Void,
            blocks: vec![],
        }
    }
}

impl<'a> Deref for Function<'a> {
    type Target = Vec<BasicBlock<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.blocks
    }
}

impl DerefMut for Function<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.blocks
    }
}

#[derive(Debug)]
pub struct IrProgram<'a> {
    pub glob: Function<'a>,
    pub funcs: HashMap<String, Function<'a>>,
    pub store: InstructionStore,
}

impl Default for IrProgram<'_> {
    fn default() -> Self {
        Self {
            glob: Function {
                name: "global".to_string(),
                arg_count: 0,
                ret_type: RegType::Void,
                blocks: vec![],
            },
            funcs: HashMap::new(),
            store: InstructionStore::default(),
        }
    }
}
