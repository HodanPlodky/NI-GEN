use std::collections::HashMap;

use frontend::{
    ast::{FnDef, Program, TopLevel},
    typeast::{PrimType, TypeDef},
};

use crate::{
    inst::RegType,
    ir::{FunctionBuilder, IrBuilder, IrProgram},
};

pub fn ir_compile(program: Program) -> IrProgram {
    todo!()
}

// name to addr
type Env = HashMap<String, u64>;

struct IrCompiler {
    env: Vec<Env>,
    ir_builer: IrBuilder,
}

impl Default for IrCompiler {
    fn default() -> Self {
        Self {
            env: vec![HashMap::new()],
            ir_builer: IrBuilder::default(),
        }
    }
}

impl From<TypeDef> for RegType {
    fn from(t: TypeDef) -> Self {
        match t {
            TypeDef::Void => RegType::Void,
            TypeDef::PrimType(PrimType::Char) => RegType::Char,
            _ => RegType::Int,
        }
    }
}

impl IrCompiler {
    fn compile(&mut self, prog: Program) -> IrProgram {
        for top in prog.items {
            match top {
                TopLevel::Function(fn_def) => self.function(fn_def),
                TopLevel::Var(_) => todo!(),
                TopLevel::Structure(_) => todo!(),
            }
        }
        let builder = std::mem::take(&mut self.ir_builer);
        builder.create()
    }

    fn function(&mut self, func: FnDef) {
        if let Some(body) = &func.body {
            let fn_b = FunctionBuilder::new(
                func.header.params.len() as u64,
                func.header.ret_type.clone().into(),
            );

        }
    }
}
