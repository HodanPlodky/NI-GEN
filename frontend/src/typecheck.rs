use std::collections::HashMap;

use crate::{
    ast::{
        AstNode, Expr, ExprType, FnDecl, FnDef, FnDefType, FnType, PrimType, Program, Statement,
        StatementType, TypeDef, Val, VarDecl, VarDeclType,
    },
    errors::{FrontendError, TypeError},
};

struct EnvLevel {
    env: HashMap<String, TypeDef>,
    ret: Option<TypeDef>,
}

impl EnvLevel {
    fn new(ret: Option<TypeDef>) -> Self {
        Self {
            env: HashMap::new(),
            ret,
        }
    }

    fn add_var(&mut self, name: &String, var_type: TypeDef) {
        self.env.insert(name.to_string(), var_type);
    }

    fn get_type(&self, name: &String) -> Option<TypeDef> {
        self.env.get(name).cloned()
    }
}

pub struct TypeData {
    type_map: HashMap<String, TypeDef>,
    env: Vec<EnvLevel>,
}

impl Default for TypeData {
    fn default() -> Self {
        Self {
            type_map: HashMap::new(),
            env: vec![EnvLevel::new(None)],
        }
    }
}

impl TypeData {
    fn add_var(&mut self, name: &String, var_type: TypeDef) -> Result<(), FrontendError> {
        if let Ok(_) = self.get_ident_type(name) {
            return Err(TypeError::IdentAlreadyExists(name.to_string()).into());
        }
        let last_index = self.env.len() - 1;
        self.env[last_index].add_var(name, var_type);
        Ok(())
    }

    fn ret(&self) -> Option<TypeDef> {
        let last_index = self.env.len() - 1;
        self.env[last_index].ret.clone()
    }

    fn get_ident_type(&self, name: &String) -> Result<TypeDef, FrontendError> {
        for e in self.env.iter().rev() {
            if let Some(t) = e.get_type(name) {
                return Ok(t);
            }
        }
        Err(TypeError::IdentDoesNotExist(name.clone()).into())
    }

    fn push_env(&mut self) {
        self.env.push(EnvLevel::new(self.ret()))
    }

    fn push_fn(&mut self, ret_type: TypeDef) {
        self.env.push(EnvLevel::new(Some(ret_type)))
    }

    fn pop_env(&mut self) {
        self.env.pop();
    }
}

type Typed = Result<TypeDef, FrontendError>;

trait TypecheckAst<T>
where
    T: PartialEq + Eq + Clone,
{
    fn typecheck(&self, data: &mut TypeData) -> Result<T, FrontendError>;
}

impl TypecheckAst<Expr> for Expr {
    fn typecheck(&self, data: &mut TypeData) -> Result<Expr, FrontendError> {
        match &self.value {
            ExprType::BinOp(_, _, _) => todo!(),
            ExprType::UnaryPreOp(_, _) => todo!(),
            ExprType::UnaryPostOp(_, _) => todo!(),
            ExprType::Value(v) => match v {
                Val::Integer(_) => Ok(self.typed(PrimType::Int.into())),
                Val::Char(_) => Ok(self.typed(PrimType::Char.into())),
            },
            ExprType::Ident(ident) => {
                let t = data.get_ident_type(ident)?;
                Ok(self.typed(t))
            }
            ExprType::Call(_, _) => todo!(),
            ExprType::Index(_, _) => todo!(),
            ExprType::Deref(e) => {
                let tmp = e.typecheck(data)?;
                let t = if let TypeDef::PointerType(t) = tmp.get_type() {
                    Ok::<Box<TypeDef>, FrontendError>(t)
                } else {
                    Err(TypeError::NonPointerDeref.into())
                }?;
                let res = ExprType::Deref(Box::new(tmp));
                let res = Expr::new(res, self.data.clone()).typed(*t);
                Ok(res)
            }
            ExprType::Address(e) => {
                let tmp = e.typecheck(data)?;
                let res = ExprType::Address(Box::new(tmp.clone()));
                let res = Expr::new(res, self.data.clone()).typed(TypeDef::PointerType(Box::new(
                    tmp.data.node_type.clone().unwrap(),
                )));
                Ok(res)
            }
            ExprType::Cast(t, _) => Ok(self.typed(t.clone())),
        }
    }
}

impl TypecheckAst<VarDecl> for VarDecl {
    fn typecheck(&self, data: &mut TypeData) -> Result<VarDecl, FrontendError> {
        let t = self.var_type.clone();

        data.add_var(&self.name, t.clone())?;

        if let Some(x) = self.init_val.clone() {
            let v_data = self.data.clone();
            let init_t = x.typecheck(data)?;

            if init_t.get_type() != t {
                return Err(TypeError::VariableTypeError(
                    self.value.name.clone(),
                    t,
                    init_t.data.node_type.unwrap(),
                )
                .into());
            }

            let res = VarDeclType {
                name: self.name.clone(),
                init_val: Some(init_t),
                var_type: self.var_type.clone(),
            };
            Ok(VarDecl::new(res, v_data).typed(TypeDef::Void))
        } else {
            Ok(self.typed(TypeDef::Void))
        }
    }
}

impl TypecheckAst<Statement> for Statement {
    fn typecheck(&self, data: &mut TypeData) -> Result<Statement, FrontendError> {
        match &self.value {
            StatementType::Expr(e) => Ok(e.typecheck(data)?.into()),
            StatementType::VarDecl(v) => Ok(v.typecheck(data)?.into()),
            StatementType::Block(stmts) => {
                let mut n_stms = vec![];
                data.push_env();
                for s in stmts {
                    n_stms.push(s.typecheck(data)?);
                }
                data.pop_env();
                let res = StatementType::Block(n_stms);
                let res = Statement::new(res, self.data.clone()).typed(TypeDef::Void);
                Ok(res)
            }
            StatementType::If(_, _) => todo!(),
            StatementType::IfElse(_, _, _) => todo!(),
            StatementType::For(_, _, _, _) => todo!(),
            StatementType::While(_, _) => todo!(),
            StatementType::Break | StatementType::Continue => Ok(self.typed(TypeDef::Void)),
            StatementType::Return(e) => match (e, data.ret()) {
                (None, None) => Ok(self.typed(TypeDef::Void)),
                (None, Some(_)) => Err(TypeError::ExpectingRet.into()),
                (Some(_), None) => Err(TypeError::UnexpectedRet.into()),
                (Some(res), Some(exp)) => {
                    let res_typed = res.typecheck(data)?;

                    if res_typed.data.node_type != Some(exp.clone()) {
                        return Err(TypeError::ReturnTypeError(
                            res_typed.data.node_type.unwrap(),
                            exp,
                        )
                        .into());
                    }

                    let res = StatementType::Return(Some(Box::new(res_typed)));
                    let res = Statement::new(res, self.data.clone()).typed(TypeDef::Void);
                    return Ok(res);
                }
            },
        }
    }
}

impl TypecheckAst<FnDef> for FnDef {
    fn typecheck(&self, data: &mut TypeData) -> Result<FnDef, FrontendError> {
        let header = self.value.header.typecheck(data)?;
        data.push_fn(header.ret_type.clone());
        let body = self.value.body.typecheck(data)?;
        data.pop_env();
        let res = FnDefType { header, body };
        let res = FnDef::new(res, self.data.clone()).typed(TypeDef::Void);
        Ok(res)
    }
}

impl TypecheckAst<FnDecl> for FnDecl {
    fn typecheck(&self, data: &mut TypeData) -> Result<FnDecl, FrontendError> {
        let t: FnType = self.clone().into();
        data.add_var(&self.name, t.into())?;
        Ok(self.typed(TypeDef::Void))
    }
}

impl TypeData {
    fn get_type(&self, symbol: String) -> Option<TypeDef> {
        self.type_map.get(&symbol).cloned()
    }
}

pub fn type_program(program: Program) -> Result<Program, FrontendError> {
    let mut data = TypeData::default();
    let mut res: Program = Program::default();
    for var in program.var_decls {
        res.var_decls.push(var.typecheck(&mut data)?);
    }

    for fn_def in program.fn_defs {
        res.fn_defs.push(fn_def.typecheck(&mut data)?);
    }

    if let Some(main) = program.main {
        res.main = Some(main.typecheck(&mut data)?);
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    fn type_ok(input: String) {
        let lex = Lexer::new("tmp".to_string(), input.chars().peekable());
        let mut parser = Parser::new(lex).unwrap();
        let res = parser.parse().unwrap();
        let typed = type_program(res);
        println!("{:?}", typed);
        assert!(typed.is_ok());
    }

    #[test]
    fn test_typedef() {
        type_ok("int main() {}".to_string());
        type_ok("int main() {return 1;}".to_string());
        type_ok("int main() {int x = 5;}".to_string());
        type_ok("int main() {int x = 5; return x;}".to_string());
    }
}
