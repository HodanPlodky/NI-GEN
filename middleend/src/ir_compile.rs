use std::collections::HashMap;

use frontend::{
    ast::{
        Expr, ExprType, FnDef, Operator, Program, Statement, StatementType, TopLevel, Val, VarDecl,
    },
    typeast::{PrimType, TypeDef},
};

use crate::{
    inst::{
        ImmC, ImmI, Reg, RegReg, RegRegs, RegType, Register, SymRegs, Terminator,
        TerminatorBranch, TerminatorJump, TerminatorReg,
    },
    ir::{FunctionBuilder, IrBuilder, IrBuilderError, IrProgram, I},
};

pub fn ir_compile(program: Program) -> Result<IrProgram, IrCompErr> {
    let mut compiler = IrCompiler::default();
    compiler.compile(program)
}

// name to register with address
type Env = HashMap<String, Register>;

#[derive(Debug)]
pub enum IrCompErr {
    Builder(IrBuilderError),
    NonExistingVar(String),
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
        self.ir_builder.add(I::Exit(Terminator), RegType::Void);

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
                    Operator::Add => Ok(f_b.add(I::Add(rr), expr.get_type().into())),
                    Operator::Sub => Ok(f_b.add(I::Sub(rr), expr.get_type().into())),
                    Operator::Mul => Ok(f_b.add(I::Mul(rr), expr.get_type().into())),
                    Operator::Div => Ok(f_b.add(I::Div(rr), expr.get_type().into())),
                    Operator::Mod => Ok(f_b.add(I::Mod(rr), expr.get_type().into())),
                    Operator::Lt => Ok(f_b.add(I::Lt(rr), expr.get_type().into())),
                    Operator::Le => Ok(f_b.add(I::Le(rr), expr.get_type().into())),
                    Operator::Gt => Ok(f_b.add(I::Gt(rr), expr.get_type().into())),
                    Operator::Ge => Ok(f_b.add(I::Ge(rr), expr.get_type().into())),
                    Operator::Eql => Ok(f_b.add(I::Eql(rr), expr.get_type().into())),
                    Operator::Neq => todo!(),
                    Operator::Assign => todo!(),
                    Operator::BitOr => Ok(f_b.add(I::Or(rr), expr.get_type().into())),
                    Operator::Or => Ok(f_b.add(I::Or(rr), expr.get_type().into())),
                    Operator::BitAnd => Ok(f_b.add(I::And(rr), expr.get_type().into())),
                    Operator::And => Ok(f_b.add(I::And(rr), expr.get_type().into())),
                    Operator::Not => todo!(),
                    Operator::BitNot => todo!(),
                    Operator::ShiftLeft => Ok(f_b.add(I::Shl(rr), expr.get_type().into())),
                    Operator::ShiftRight => Ok(f_b.add(I::Shr(rr), expr.get_type().into())),
                    _ => unreachable!(),
                }
            }

            ExprType::UnaryPreOp(op, expr) => {
                let _reg = self.compile_expr(expr, f_b)?;
                let _addition = match op {
                    Operator::Inc => f_b.add(I::Ldi(ImmI(1)), RegType::Int),
                    Operator::Dec => f_b.add(I::Ldi(ImmI(-1)), RegType::Int),
                    _ => unreachable!(),
                };
                //Ok(f_b.add(I::Add(RegReg(reg, addition)), RegType::Int));
                todo!()
            }
            ExprType::UnaryPostOp(_, _) => todo!(),
            ExprType::Value(v) => self.compile_val(v, f_b),
            ExprType::Ident(name) => {
                let reg = self.get_addreg(name.clone())?;
                Ok(f_b.add(I::Ld(Reg(reg)), expr.get_type().into()))
            }
            ExprType::Call(target, args) => {
                let mut args_regs: Vec<Register> = vec![];
                for arg in args {
                    args_regs.push(self.compile_expr(arg, f_b)?);
                }
                match &target.value {
                    ExprType::Ident(name) if self.get_addreg(name.clone()).is_err() => Ok(f_b.add(
                        I::CallDirect(SymRegs(name.clone(), args_regs)),
                        expr.get_type().into(),
                    )),
                    _ => {
                        let target = self.compile_expr(target, f_b)?;
                        Ok(f_b.add(I::Call(RegRegs(target, args_regs)), expr.get_type().into()))
                    }
                }
            }
            ExprType::Index(_, _) => todo!(),
            ExprType::Deref(pointer) => {
                let reg = self.compile_expr(pointer, f_b)?;
                Ok(f_b.add(I::Ld(Reg(reg)), expr.get_type().into()))
            }
            ExprType::Address(e) => match &e.value {
                ExprType::Ident(name) => self.get_addreg(name.clone()),
                _ => todo!(),
            },
            ExprType::Cast(_, _) => todo!(),
            ExprType::FieldAccess(_, _) => todo!(),
        }
    }

    fn get_addreg(&mut self, name: String) -> Result<Register, IrCompErr> {
        for i in (0..self.env.len()).rev() {
            if let Some(reg) = self.env[i].get(&name) {
                return Ok(*reg);
            }
        }
        Err(IrCompErr::NonExistingVar(name))
    }

    fn compile_named_assign(
        &mut self,
        name: String,
        expr: &Expr,
        f_b: &mut FunctionBuilder,
    ) -> Result<(), IrCompErr> {
        let reg_store = self.get_addreg(name)?;
        let reg_val = self.compile_expr(expr, f_b)?;
        f_b.add(I::St(RegReg(reg_store, reg_val)), RegType::Void);
        Ok(())
    }

    fn compile_assign(
        &mut self,
        store: &Expr,
        expr: &Expr,
        f_b: &mut FunctionBuilder,
    ) -> Result<(), IrCompErr> {
        let reg_store = match &store.value {
            ExprType::Ident(name) => self.get_addreg(name.clone()),
            ExprType::Deref(e) => self.compile_expr(e, f_b),
            _ => todo!(),
        }?;
        let reg_val = self.compile_expr(expr, f_b)?;
        f_b.add(I::St(RegReg(reg_store, reg_val)), RegType::Void);
        Ok(())
    }

    fn compile_vardecl(
        &mut self,
        decl: &VarDecl,
        f_b: &mut FunctionBuilder,
    ) -> Result<(), IrCompErr> {
        let size = match decl.value.var_type {
            TypeDef::Void => unreachable!(),
            TypeDef::PrimType(PrimType::Int) => 8,
            TypeDef::PrimType(PrimType::Char) => 1,
            TypeDef::PointerType(_) => 8,
            TypeDef::Function(_) => todo!(),
            TypeDef::Alias(_) => todo!(),
            TypeDef::Struct(_) => todo!(),
            TypeDef::Array(_) => todo!(),
        };
        let reg = f_b.add(I::Alloca(ImmI(size)), RegType::Int);
        self.env.last_mut().unwrap().insert(decl.name.clone(), reg);
        if let Some(init_val) = &decl.value.init_val {
            self.compile_named_assign(decl.name.clone(), init_val, f_b)?;
        }
        Ok(())
    }

    fn compile_stmt(
        &mut self,
        stmt: &Statement,
        f_b: &mut FunctionBuilder,
    ) -> Result<(), IrCompErr> {
        match &stmt.value {
            StatementType::Expr(e) => match &e.value {
                ExprType::BinOp(Operator::Assign, l, r) => self.compile_assign(l, r, f_b)?,
                _ => _ = self.compile_expr(e, f_b)?,
            },
            StatementType::VarDecl(decl) => self.compile_vardecl(decl, f_b)?,
            StatementType::Block(stmts) => {
                for s in stmts {
                    self.compile_stmt(s, f_b)?;
                }
            }
            StatementType::If(guard, block) => {
                let guard_reg = self.compile_expr(guard, f_b)?;
                let then = f_b.create_bb();
                let after = f_b.create_bb();
                f_b.add(
                    I::Branch(TerminatorBranch(guard_reg, then, after)),
                    RegType::Void,
                );
                f_b.set_bb(then);
                self.compile_stmt(block, f_b)?;
                if !f_b.terminated() {
                    f_b.add(I::Jmp(TerminatorJump(after)), RegType::Void);
                }
                f_b.set_bb(after);
            }
            StatementType::IfElse(guard, then_block, else_block) => {
                let guard_reg = self.compile_expr(guard, f_b)?;
                let then_bb = f_b.create_bb();
                let else_bb = f_b.create_bb();
                let after = f_b.create_bb();
                f_b.add(
                    I::Branch(TerminatorBranch(guard_reg, then_bb, else_bb)),
                    RegType::Void,
                );
                f_b.set_bb(then_bb);
                self.compile_stmt(then_block, f_b)?;
                if !f_b.terminated() {
                    f_b.add(I::Jmp(TerminatorJump(after)), RegType::Void);
                }
                f_b.set_bb(else_bb);
                self.compile_stmt(else_block, f_b)?;
                if !f_b.terminated() {
                    f_b.add(I::Jmp(TerminatorJump(after)), RegType::Void);
                }
                f_b.set_bb(after);
            }
            StatementType::For(init, guard, after, body) => {
                let check_bb = f_b.create_bb();
                let body_bb = f_b.create_bb();
                let after_bb = f_b.create_bb();
                if let Some(init) = init {
                    self.compile_stmt(init, f_b)?;
                }

                f_b.set_bb(check_bb);
                let guard_reg = if let Some(guard) = guard {
                    self.compile_expr(guard, f_b)
                } else {
                    Ok(f_b.add(I::Ldi(ImmI(1)), RegType::Int))
                }?;
                f_b.add(
                    I::Branch(TerminatorBranch(guard_reg, body_bb, after_bb)),
                    RegType::Void,
                );

                f_b.set_bb(body_bb);
                self.compile_stmt(body, f_b)?;
                if let Some(after) = after {
                    self.compile_stmt(after, f_b)?;
                }
                f_b.add(I::Jmp(TerminatorJump(check_bb)), RegType::Void);

                f_b.set_bb(after_bb);
            }
            StatementType::While(guard, body) => {
                let check_bb = f_b.create_bb();
                f_b.add(I::Jmp(TerminatorJump(check_bb)), RegType::Void);
                f_b.set_bb(check_bb);
                let guard_reg = self.compile_expr(guard, f_b)?;
                let body_bb = f_b.create_bb();
                let after_bb = f_b.create_bb();
                f_b.add(
                    I::Branch(TerminatorBranch(guard_reg, body_bb, after_bb)),
                    RegType::Void,
                );
                f_b.set_bb(body_bb);
                self.compile_stmt(body, f_b)?;
                f_b.add(I::Jmp(TerminatorJump(check_bb)), RegType::Void);
                f_b.set_bb(after_bb);
            }
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

            for index in 0..func.header.params.len() {
                let t: RegType = func.header.params[index].1.clone().into();
                let reg = fn_b.add(I::Arg(ImmI(index as i64)), t);
                let addr = fn_b.add(I::Alloca(ImmI(8)), RegType::Int);
                fn_b.add(I::St(RegReg(addr, reg)), RegType::Void);
                self.env
                    .last_mut()
                    .unwrap()
                    .insert(func.header.params[index].0.clone(), addr);
            }

            self.compile_stmt(body, &mut fn_b)?;
            if !fn_b.terminated() {
                fn_b.add(I::Ret(Terminator), RegType::Void);
            }
            self.ir_builder.add_fn(fn_b.create(&func.header.name))?;
        }
        Ok(())
    }
}
