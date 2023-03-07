use std::collections::HashMap;

use crate::{
    ast::{AstNode, Expr, PrimType, Program, Statement, TypeDef, Val, VarDecl, VarDeclType},
    errors::FrontendError,
};

pub struct Typechecker {
    type_map: HashMap<String, TypeDef>,
}

impl Default for Typechecker {
    fn default() -> Self {
        todo!()
    }
}

type Typed = Result<TypeDef, FrontendError>;

trait TypecheckAst<T>
where
    T: PartialEq + Eq + Clone,
{
    fn typecheck(&self) -> Result<T, FrontendError>;
}

impl TypecheckAst<Expr> for Expr {
    fn typecheck(&self) -> Result<Expr, FrontendError> {
        match self.value {
            crate::ast::ExprType::BinOp(_, _, _) => todo!(),
            crate::ast::ExprType::UnaryPreOp(_, _) => todo!(),
            crate::ast::ExprType::UnaryPostOp(_, _) => todo!(),
            crate::ast::ExprType::Value(_) => todo!(),
            crate::ast::ExprType::Ident(_) => todo!(),
            crate::ast::ExprType::Call(_, _) => todo!(),
            crate::ast::ExprType::Index(_, _) => todo!(),
            crate::ast::ExprType::Deref(_) => todo!(),
            crate::ast::ExprType::Address(_) => todo!(),
            crate::ast::ExprType::Cast(_, _) => todo!(),
        }
    }
}

impl TypecheckAst<VarDecl> for VarDecl {
    fn typecheck(&self) -> Result<VarDecl, FrontendError> {
        let t = self.var_type.clone();
        if let Some(x) = self.init_val.clone() {
            let data = self.data.clone();
            let init_t = x.typecheck()?;



            let res = VarDeclType {
                name: self.name.clone(),
                init_val: Some(init_t),
                var_type: self.var_type.clone(),
            };
            Ok(VarDecl::new(res, data))
        } else {
            Ok(self.typed(TypeDef::Void))
        }
    }
}

impl TypecheckAst<Statement> for Statement {
    fn typecheck(&self) -> Result<Statement, FrontendError> {
        match &self.value {
            crate::ast::StatementType::Expr(e) => Ok(e.typecheck()?.into()),
            crate::ast::StatementType::VarDecl(v) => Ok(v.typecheck()?.into()),
            crate::ast::StatementType::Block(_) => todo!(),
            crate::ast::StatementType::If(_, _) => todo!(),
            crate::ast::StatementType::IfElse(_, _, _) => todo!(),
            crate::ast::StatementType::For(_, _, _, _) => todo!(),
            crate::ast::StatementType::While(_, _) => todo!(),
            crate::ast::StatementType::Break => todo!(),
            crate::ast::StatementType::Continue => todo!(),
            crate::ast::StatementType::Return(_) => todo!(),
        }
    }
}

impl Typechecker {
    fn get_type(&self, symbol: String) -> Option<TypeDef> {
        self.type_map.get(&symbol).cloned()
    }

    pub fn type_program(&mut self, prog: Program) -> Result<(), FrontendError> {
        todo!()
    }
}
