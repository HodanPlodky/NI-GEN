use std::collections::HashMap;

use frontend::{
    ast::{Expr, FnDef, Program, Statement, StatementType, TopLevel},
    typeast::{PrimType, TypeDef},
};

use crate::{
    inst::{RegType, Register, Terminator, TerminatorReg},
    ir::{FunctionBuilder, IrBuilder, IrBuilderError, IrProgram, I},
};

pub fn ir_compile(program: Program) -> Result<IrProgram, IrCompErr> {
    let mut compiler = IrCompiler::default();
    compiler.compile(program)
}

// name to addr
type Env = HashMap<String, u64>;

#[derive(Debug)]
pub enum IrCompErr {
    Builder(IrBuilderError),
    Unknown,
}

impl From<IrBuilderError> for IrCompErr {
    fn from(e: IrBuilderError) -> Self {
        IrCompErr::Builder(e)
    }
}

struct IrCompiler {
    env: Vec<Env>,
    ir_builder: IrBuilder,
}

impl Default for IrCompiler {
    fn default() -> Self {
        Self {
            env: vec![HashMap::new()],
            ir_builder: IrBuilder::default(),
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
    fn compile(&mut self, prog: Program) -> Result<IrProgram, IrCompErr> {
        for top in prog.items {
            match top {
                TopLevel::Function(fn_def) => self.function(fn_def)?,
                TopLevel::Var(_) => todo!(),
                TopLevel::Structure(_) => todo!(),
            }
        }
        let builder = std::mem::take(&mut self.ir_builder);
        Ok(builder.create())
    }

    fn compile_expr(
        &mut self,
        expr: &Expr,
        f_b: &mut FunctionBuilder,
    ) -> Result<Register, IrCompErr> {
        todo!()
    }

    fn compile_stmt(
        &mut self,
        stmt: &Statement,
        f_b: &mut FunctionBuilder,
    ) -> Result<(), IrCompErr> {
        match &stmt.value {
            StatementType::Expr(_) => todo!(),
            StatementType::VarDecl(_) => todo!(),
            StatementType::Block(stmts) => {
                for s in stmts {
                    self.compile_stmt(s, f_b)?;
                }
            }
            StatementType::If(_, _) => todo!(),
            StatementType::IfElse(_, _, _) => todo!(),
            StatementType::For(_, _, _, _) => todo!(),
            StatementType::While(_, _) => todo!(),
            StatementType::Break => todo!(),
            StatementType::Continue => todo!(),
            StatementType::Return(Some(expr)) => {
                let reg = self.compile_expr(expr, f_b)?;
                f_b.add(I::Retr(TerminatorReg(reg)), RegType::Void);
            }
            StatementType::Return(None) => {
                f_b.add(I::Ret(Terminator), RegType::Void);
            }
        }
        Ok(())
    }

    fn function(&mut self, func: FnDef) -> Result<(), IrCompErr> {
        if let Some(body) = &func.body {
            let mut fn_b = FunctionBuilder::new(
                func.header.params.len() as u64,
                func.header.ret_type.clone().into(),
            );

            self.compile_stmt(body, &mut fn_b)?;
            self.ir_builder.add_fn(fn_b.create(&func.header.name))?;
        }
        Ok(())
    }
}
