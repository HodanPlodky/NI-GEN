use std::collections::HashMap;

use frontend::{
    ast::{Expr, ExprType, FnDef, Program, Statement, StatementType, TopLevel, Val},
    typeast::{PrimType, TypeDef},
};

use crate::{
    inst::{ImmC, ImmI, RegType, Register, Terminator, TerminatorReg, RegReg},
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
        self.ir_builder.add(I::Ret(Terminator), RegType::Void);
        let builder = std::mem::take(&mut self.ir_builder);
        Ok(builder.create())
    }

    fn compile_val(&mut self, val: &Val, f_b: &mut FunctionBuilder) -> Result<Register, IrCompErr> {
        match val {
            Val::Integer(num) => Ok(f_b.add(I::Ldi(ImmI(*num)), RegType::Int)),
            Val::Char(c) => Ok(f_b.add(I::Ldc(ImmC(*c)), RegType::Char)),
        }
    }

    fn compile_expr(
        &mut self,
        expr: &Expr,
        f_b: &mut FunctionBuilder,
    ) -> Result<Register, IrCompErr> {
        match &expr.value {
            ExprType::BinOp(op, l, r) => {
                let l_reg = self.compile_expr(l, f_b)?;
                let r_reg = self.compile_expr(r, f_b)?;
                let rr = RegReg(l_reg, r_reg);
                match op {
                    frontend::ast::Operator::Add => Ok(f_b.add(I::Add(rr), expr.get_type().into())),
                    frontend::ast::Operator::Sub => Ok(f_b.add(I::Sub(rr), expr.get_type().into())),
                    frontend::ast::Operator::Mul => Ok(f_b.add(I::Mul(rr), expr.get_type().into())),
                    frontend::ast::Operator::Div => Ok(f_b.add(I::Div(rr), expr.get_type().into())),
                    frontend::ast::Operator::Mod => Ok(f_b.add(I::Mod(rr), expr.get_type().into())),
                    frontend::ast::Operator::Inc => todo!(),
                    frontend::ast::Operator::Dec => todo!(),
                    frontend::ast::Operator::Lt => todo!(),
                    frontend::ast::Operator::Le => todo!(),
                    frontend::ast::Operator::Gt => todo!(),
                    frontend::ast::Operator::Ge => todo!(),
                    frontend::ast::Operator::Eql => todo!(),
                    frontend::ast::Operator::Neq => todo!(),
                    frontend::ast::Operator::Assign => todo!(),
                    frontend::ast::Operator::BitOr => todo!(),
                    frontend::ast::Operator::Or => todo!(),
                    frontend::ast::Operator::BitAnd => todo!(),
                    frontend::ast::Operator::And => todo!(),
                    frontend::ast::Operator::Not => todo!(),
                    frontend::ast::Operator::BitNot => todo!(),
                    frontend::ast::Operator::ShiftLeft => todo!(),
                    frontend::ast::Operator::ShiftRight => todo!(),
                }
            },
            ExprType::UnaryPreOp(_, _) => todo!(),
            ExprType::UnaryPostOp(_, _) => todo!(),
            ExprType::Value(v) => self.compile_val(v, f_b),
            ExprType::Ident(_) => todo!(),
            ExprType::Call(_, _) => todo!(),
            ExprType::Index(_, _) => todo!(),
            ExprType::Deref(_) => todo!(),
            ExprType::Address(_) => todo!(),
            ExprType::Cast(_, _) => todo!(),
            ExprType::FieldAccess(_, _) => todo!(),
        }
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
            if !fn_b.terminated() {
                fn_b.add(I::Ret(Terminator), RegType::Void);
            }
            self.ir_builder.add_fn(fn_b.create(&func.header.name))?;
        }
        Ok(())
    }
}
