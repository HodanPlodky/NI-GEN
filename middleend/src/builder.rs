// ----------------------------------------------------------------------
// Builders
// ----------------------------------------------------------------------

use crate::{
    inst::InstructionType,
    ir::{
        BBIndex, BasicBlock, Function, InstUUID, Instruction, InstructionStore, IrProgram, RegType,
        Register,
    },
};

#[derive(Debug)]
pub struct BuildContext {
    store: InstructionStore,
}

impl Default for BuildContext {
    fn default() -> Self {
        Self {
            store: InstructionStore::default(),
        }
    }
}

#[derive(Debug)]
pub struct IrBuilder {
    global: Function,
    prog: IrProgram,
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

impl IrBuilder {
    pub fn new() -> Self {
        Self {
            global: Function {
                name: "global".to_string(),
                arg_count: 0,
                ret_type: RegType::Void,
                blocks: vec![],
            },
            prog: IrProgram::default(),
        }
    }

    fn get_id(&self) -> InstUUID {
        self.prog.store.create_id()
    }

    pub fn create_fnbuild(
        &self,
        arg_count: u64,
        ret_type: RegType,
        context: &'c BuildContext,
    ) -> FunctionBuilder<'c> {
        FunctionBuilder {
            arg_count,
            ret_type,
            act_bb: BBIndex(0),
            blocks: vec![BasicBlock::new(&context.store)],
            context,
        }
    }

    pub fn add_fn(&mut self, func: Function) -> Result<(), IrBuilderError> {
        if self.prog.funcs.contains_key(&func.name.to_string()) {
            return Err(IrBuilderError::FuncRedef);
        }
        self.prog.funcs.insert(func.name.to_string(), func);
        Ok(())
    }

    pub fn add(&mut self, inst: InstructionType, reg_type: RegType) -> Register {
        let id = self.get_id();
        let inst = Instruction::new(id, reg_type, None, inst);
        let id = self.prog.store.add_instruction(inst);
        self.global.blocks[0].push(id);
        id
    }

    pub fn add_astdata(&self, _inst: InstructionType) {
        // this is just for better comp so fuck it
        todo!()
    }

    pub fn create(self, context : BuildContext) -> IrProgram {
        let mut tmp = self;
        tmp.prog.glob = std::mem::take(&mut tmp.global);
        tmp.prog.store = context.store.clone();
        tmp.prog
    }
}

pub struct FunctionBuilder<'a> {
    context: &'a BuildContext,
    arg_count: u64,
    ret_type: RegType,
    act_bb: BBIndex,
    blocks: Vec<BasicBlock>,
}

impl<'a> FunctionBuilder<'a> {
    fn get_id(&self) -> InstUUID {
        self.context.store.create_id()
    }

    pub fn create_bb(&mut self) -> BBIndex {
        self.blocks.push(BasicBlock::new(&self.context.store));
        BBIndex(self.blocks.len() - 1)
    }

    pub fn add(&mut self, inst: InstructionType, reg_type: RegType) -> Register {
        let id = self.get_id();
        let inst = Instruction::new(id, reg_type, None, inst);
        let id = unsafe {
            let tmp = &self.context.store as *const InstructionStore as *mut InstructionStore;
            tmp.as_mut().unwrap().add_instruction(inst)
        };
        self.blocks[self.act_bb.index()].push(id);
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

    pub fn create(self, name: &str) -> Function {
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
        let context = BuildContext::default();
        let mut builder = IrBuilder::new(&context);
        let reg: Register = builder.add(InstructionType::Ldi(ImmI(5)), RegType::Int);
        builder.add(InstructionType::Ret(Terminator), RegType::Void);
        let func = {
            let mut fn_b = builder.create_fnbuild(0, RegType::Void, &context);
            let bi = fn_b.create_bb();
            fn_b.add(InstructionType::Jmp(TerminatorJump(bi)), RegType::Void);
            fn_b.set_bb(bi);
            fn_b.add(InstructionType::Print(Reg(reg)), RegType::Void);
            fn_b.add(InstructionType::Ret(Terminator), RegType::Void);
            fn_b.create("main")
        };
        builder.add_fn(func).unwrap();
    }
}
