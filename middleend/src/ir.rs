use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

//use frontend::ast::AstData;

use crate::inst::{InstructionType, TerminatorBranch, TerminatorJump};

/// Id of the instruction
/// it is the index into the instruction store
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub struct InstUUID(usize);

impl InstUUID {
    pub fn val(&self) -> usize {
        self.0
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Instruction {
    pub id: InstUUID,
    pub reg_type: RegType,
    pub data: InstructionType,
}

impl Instruction {
    pub fn new(id: InstUUID, reg_type: RegType, data: InstructionType) -> Self {
        Self { id, reg_type, data }
    }
}

impl From<Instruction> for Register {
    fn from(value: Instruction) -> Self {
        return value.id;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    pub instruction: Vec<InstUUID>,
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
    type Target = Vec<InstUUID>;

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

    pub fn succ(&self, store: &InstStore) -> Vec<BBIndex> {
        use InstructionType::*;
        let inst = match self.last() {
            Some(inst) => inst,
            None => return vec![],
        };
        match &store.get(*inst).data {
            Jmp(TerminatorJump(bbindex)) => vec![*bbindex],
            Branch(TerminatorBranch(_, bbindex_true, bbindex_false)) => {
                vec![*bbindex_true, *bbindex_false]
            }
            _ => [].to_vec(),
        }
    }

    pub fn get_used_regs(&self, store: &InstStore) -> Vec<Register> {
        self.iter()
            .map(|x| store.get(*x).data.get_regs())
            .flatten()
            .collect()
    }

    pub fn terminated(&self, store: &InstStore) -> bool {
        if self.is_empty() {
            false
        } else {
            store.get(*self.last().unwrap()).data.terminator()
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

    pub fn get_used_regs(&self, store: &InstStore) -> Vec<Register> {
        self.iter()
            .map(|x| x.get_used_regs(store))
            .flatten()
            .collect()
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

#[derive(Default, Debug)]
pub struct InstStore {
    data: Vec<Instruction>,
}

impl Deref for InstStore {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl InstStore {
    pub fn get_next_id(&self) -> InstUUID {
        InstUUID(self.data.len())
    }

    pub fn add_inst(&mut self, inst: InstructionType, reg_type: RegType) -> InstUUID {
        let id = self.get_next_id();
        let inst = Instruction::new(id, reg_type, inst);
        self.data.push(inst);
        id
    }

    pub fn replace_inst(
        &mut self,
        id: InstUUID,
        inst: InstructionType,
        reg_type: RegType,
    ) -> InstUUID {
        let inst = Instruction::new(id, reg_type, inst);
        self.data[id.0] = inst;
        id
    }

    pub fn get(&self, id: InstUUID) -> &Instruction {
        &self.data[id.0]
    }

    pub fn get_mut(&mut self, id: InstUUID) -> &mut Instruction {
        &mut self.data[id.0]
    }
}

#[derive(Debug)]
pub struct IrProgram {
    pub store: InstStore,
    pub glob: Function,
    pub funcs: HashMap<String, Function>,
}

impl Default for IrProgram {
    fn default() -> Self {
        Self {
            store: InstStore::default(),
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

impl IrProgram {
    pub fn get_type(&self, reg: Register) -> RegType {
        self.store[reg.0].reg_type
    }

    pub fn get_inst(&self, inst_id: InstUUID) -> &Instruction {
        &self.store[inst_id.0]
    }
}
