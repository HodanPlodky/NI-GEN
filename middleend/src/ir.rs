use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::inst::{BBIndex, BasicBlock, InstUUID, Instruction, InstructionType, RegType, Register};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    pub arg_count: u64,
    pub ret_type: RegType,
    pub blocks: Vec<BasicBlock>,
}

impl Function {
    pub fn start(&self) -> &BasicBlock {
        &self.blocks[0]
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
    global: BasicBlock,
    act_bb: BBIndex,
    act_fn: Option<String>,
    prog: IrProgram,
}

impl Default for IrBuilder {
    fn default() -> Self {
        Self {
            global: BasicBlock::default(),
            act_bb: 0,
            act_fn: None,
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
    CannotCreateId,
    FuncRedef,
}

// for better writing
pub type I = InstructionType;

impl IrBuilder {
    fn get_id(&self) -> InstUUID {
        (true, 0, self.global.len())
    }

    pub fn add_fn(&mut self, name: &str, func: Function) -> Result<(), IrBuilderError> {
        if self.prog.funcs.contains_key(&name.to_string()) {
            return Err(IrBuilderError::FuncRedef);
        }
        self.prog.funcs.insert(name.to_string(), func);
        Ok(())
    }

    pub fn add(&mut self, inst: InstructionType, reg_type: RegType) -> Register {
        let id = self.get_id();
        let inst = Instruction::new(id, reg_type, None, inst);
        self.global.push(inst);
        id
    }

    pub fn add_astdata(&self, _inst: InstructionType) {
        // this is just for better comp so fuck it
        todo!()
    }

    pub fn create(self) -> IrProgram {
        let mut tmp = self;
        tmp.prog.glob = std::mem::take(&mut tmp.global);
        tmp.prog
    }
}

pub struct FunctionBuilder {
    arg_count: u64,
    ret_type: RegType,
    act_bb: BBIndex,
    blocks: Vec<BasicBlock>,
}

impl FunctionBuilder {
    pub fn new(arg_count: u64, ret_type: RegType) -> Self {
        Self {
            arg_count,
            ret_type,
            act_bb: 0,
            blocks: vec![BasicBlock::default()],
        }
    }

    pub fn get_id(&self) -> InstUUID {
        (false, self.act_bb, self.blocks[self.act_bb].len())
    }

    pub fn create_bb(&mut self) -> BBIndex {
        self.blocks.push(BasicBlock::default());
        self.blocks.len() - 1
    }

    pub fn add(&mut self, inst: InstructionType, reg_type: RegType) -> Register {
        let id = self.get_id();
        let inst = Instruction::new(id, reg_type, None, inst);
        self.blocks[self.act_bb].push(inst);
        id
    }

    pub fn set_bb(&mut self, bi: BBIndex) {
        self.act_bb = bi;
    }

    pub fn create(self) -> Function {
        Function {
            arg_count: self.arg_count,
            ret_type: self.ret_type,
            blocks: self.blocks,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::inst::{ImmI, Reg, Terminator, TerminatorJump};

    use super::*;

    #[test]
    fn correct_builder_api() {
        let mut builder = IrBuilder::default();
        let reg: Register = builder.add(I::Ldi(ImmI(5)), RegType::Int);
        builder.add(I::Ret(Terminator), RegType::Void);
        let mut fn_b = FunctionBuilder::new(0, RegType::Void);
        let bi = fn_b.create_bb();
        fn_b.add(I::Jmp(TerminatorJump(bi)), RegType::Void);
        fn_b.set_bb(bi);
        fn_b.add(I::Print(Reg(reg)), RegType::Void);
        fn_b.add(I::Ret(Terminator), RegType::Void);
        builder.add_fn("main", fn_b.create()).unwrap();
    }
}
