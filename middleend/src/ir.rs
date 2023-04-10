use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::inst::{BBIndex, BasicBlock, InstUUID, Instruction, InstructionType, RegType, Register};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    pub arg_count: u64,
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
}

// for better writing
pub type I = InstructionType;

impl IrBuilder {
    fn get_id(&self, glob: bool) -> Result<InstUUID, IrBuilderError> {
        if glob {
            Ok((true, 0, self.global.len()))
        } else if let Some(func) = self.act_fn.as_ref().and_then(|x| self.prog.funcs.get(x)) {
            Ok((false, self.act_bb, func[self.act_bb].len()))
        } else {
            Err(IrBuilderError::CannotCreateId)
        }
    }

    pub fn add(
        &mut self,
        inst: InstructionType,
        reg_type: RegType,
    ) -> Result<Register, IrBuilderError> {
        todo!()
    }

    pub fn create_bb(&mut self) -> Result<BBIndex, IrBuilderError> {
        todo!()
    }

    pub fn set_bb(&mut self, bi: BBIndex) -> Result<BBIndex, IrBuilderError> {
        todo!()
    }

    pub fn create_fn(
        &mut self,
        name: &str,
        arg_count: u64,
        ret_type: RegType,
    ) -> Result<(), IrBuilderError> {
        todo!()
    }

    pub fn add_global(
        &self,
        inst: InstructionType,
        reg_type: RegType,
    ) -> Result<Register, IrBuilderError> {
        todo!()
    }

    pub fn add_astdata(&self, _inst: InstructionType) {
        // this is just for better comp so fuck it
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::inst::{ImmI, Reg, Terminator, TerminatorJump};

    use super::*;

    #[test]
    fn correct_builder_api() {
        let mut builder = IrBuilder::default();
        let reg: Register = builder.add_global(I::Ldi(ImmI(5)), RegType::Int).unwrap();
        builder
            .add_global(I::Ret(Terminator), RegType::Void)
            .unwrap();
        builder.create_fn("main", 0, RegType::Void).unwrap();
        let bi = builder.create_bb().unwrap();
        builder
            .add(I::Jmp(TerminatorJump(bi)), RegType::Void)
            .unwrap();
        builder.set_bb(bi).unwrap();
        builder.add(I::Print(Reg(reg)), RegType::Void).unwrap();
        builder.add(I::Ret(Terminator), RegType::Void).unwrap();
    }
}
