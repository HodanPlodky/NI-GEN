// ----------------------------------------------------------------------
// Builders
// ----------------------------------------------------------------------

use crate::{
    inst::InstructionType,
    ir::{BBIndex, BasicBlock, Function, InstUUID, Instruction, IrProgram, RegType, Register},
};

#[derive(Debug)]
pub struct IrBuilder<'a> {
    global: Function<'a>,
    prog: IrProgram<'a>,
    insts: Vec<Instruction>,
}

impl Default for IrBuilder<'_> {
    fn default() -> Self {
        Self {
            global: Function {
                name: "global".to_string(),
                arg_count: 0,
                ret_type: RegType::Void,
                blocks: vec![BasicBlock::default()],
            },
            prog: IrProgram::default(),
            insts: vec![],
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

impl<'a> IrBuilder<'a> {
    fn get_id(&self) -> InstUUID {
        InstUUID(self.insts.len())
    }

    pub fn create_fnbuild(&'a mut self, arg_count: u64, ret_type: RegType) -> FunctionBuilder<'a> {
        FunctionBuilder {
            parent: self,
            arg_count,
            ret_type,
            act_bb: BBIndex(0),
            blocks: vec![BasicBlock::default()],
        }
    }

    pub fn add_fn(&'a mut self, func: Function<'a>) -> Result<(), IrBuilderError> {
        if self.prog.funcs.contains_key(&func.name.to_string()) {
            return Err(IrBuilderError::FuncRedef);
        }
        self.prog.funcs.insert(func.name.to_string(), func);
        Ok(())
    }

    pub fn add(&'a mut self, inst: InstructionType, reg_type: RegType) -> Register {
        let id = self.get_id();
        let inst = Instruction::new(id, reg_type, None, inst);
        self.insts.push(inst);
        self.global.blocks[0].push(self.insts.last().unwrap());
        id
    }

    pub fn add_astdata(&self, _inst: InstructionType) {
        // this is just for better comp so fuck it
        todo!()
    }

    pub fn create(self) -> IrProgram<'a> {
        let mut tmp = self;
        tmp.prog.glob = std::mem::take(&mut tmp.global);
        tmp.insts = std::mem::take(&mut tmp.insts);
        tmp.prog
    }
}

pub struct FunctionBuilder<'a> {
    parent: &'a mut IrBuilder<'a>,
    arg_count: u64,
    ret_type: RegType,
    act_bb: BBIndex,
    blocks: Vec<BasicBlock<'a>>,
}

impl<'a> FunctionBuilder<'a> {
    pub fn create_bb(&mut self) -> BBIndex {
        self.blocks.push(BasicBlock::default());
        BBIndex(self.blocks.len() - 1)
    }

    pub fn add(&'a mut self, inst: InstructionType, reg_type: RegType) -> Register {
        let id = self.parent.get_id();
        let inst = Instruction::new(id, reg_type, None, inst);
        self.parent.insts.push(inst);
        self.blocks[self.act_bb.index()].push(self.parent.insts.last().unwrap());
        id
    }

    pub fn set_bb(&mut self, bi: BBIndex) {
        self.act_bb = bi;
    }

    pub fn get_act_bb(&self) -> BBIndex {
        self.act_bb
    }

    /// needed for dataflow analysis
    pub fn set_predecesors(&mut self, to: BBIndex, preds: &[BBIndex]) {
        for pred in preds {
            self.blocks[to.index()].add_predecesor(*pred)
        }
    }

    pub fn terminated(&self) -> bool {
        self.blocks[self.act_bb.index()].terminated()
    }

    pub fn create(self, name: &'a str) -> Function {
        Function {
            name: name.to_string(),
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
        let reg: Register = builder.add(InstructionType::Ldi(ImmI(5)), RegType::Int);
        builder.add(InstructionType::Ret(Terminator), RegType::Void);
        let mut fn_b = builder.create_fnbuild(0, RegType::Void);
        let bi = fn_b.create_bb();
        fn_b.add(InstructionType::Jmp(TerminatorJump(bi)), RegType::Void);
        fn_b.set_bb(bi);
        fn_b.add(InstructionType::Print(Reg(reg)), RegType::Void);
        fn_b.add(InstructionType::Ret(Terminator), RegType::Void);
        builder.add_fn(fn_b.create("main")).unwrap();
    }
}
