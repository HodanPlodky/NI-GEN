use std::collections::HashMap;

use crate::{
    ast::{
        AstData, Expr, ExprType, FnDecl, FnDef, FnDefType, Program, Statement, StatementType,
        StructDef, StructDefType, TopLevel, Val, VarDecl, VarDeclType,
    },
    errors::{FrontendError, TypeError},
    lexer::Operator,
    typeast::{FnType, PrimType, TypeDef},
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

    fn add_force(&mut self, name: &String, var_type: TypeDef) -> Result<(), FrontendError> {
        let last_index = self.env.len() - 1;
        self.env[last_index].add_var(name, var_type);
        Ok(())
    }

    fn add_type(&mut self, name: &String, value: TypeDef) {
        self.type_map.insert(name.clone(), value);
    }

    fn get_type(&mut self, name: &String) -> Result<TypeDef, FrontendError> {
        Ok(self.type_map[name].clone())
    }

    fn translate_type(&mut self, type_def: TypeDef) -> Result<TypeDef, FrontendError> {
        match type_def {
            TypeDef::Alias(name) => self.get_type(&name),
            t => Ok(t),
        }
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

trait TypecheckAst<T>
where
    T: PartialEq + Eq + Clone,
{
    fn typecheck(&self, data: &mut TypeData) -> Result<T, FrontendError>;
}

fn unary_op(
    op: Operator,
    expr: Box<Expr>,
    data: &mut TypeData,
) -> Result<(Expr, TypeDef), FrontendError> {
    let expr = expr.typecheck(data)?;
    let t = match (op, expr.get_type()) {
        (Operator::Not, TypeDef::PrimType(PrimType::Int)) => PrimType::Int.into(),
        (_, TypeDef::PrimType(t)) => t.into(),
        (_, TypeDef::PointerType(t)) => TypeDef::PointerType(t),
        _ => return Err(TypeError::InvalidOperation(op).into()),
    };

    Ok((expr, t))
}

fn assign(left: &Expr, right: &Expr) -> Result<TypeDef, FrontendError> {
    match (left.value.clone(), right.value.clone()) {
        (ExprType::Ident(_), _) => (),
        (ExprType::Deref(_), _) => (),
        _ => return Err(TypeError::CannotAssignInto(left.clone()).into()),
    }
    Ok(TypeDef::Void)
}

fn binary_op(
    op: Operator,
    left: Box<Expr>,
    right: Box<Expr>,
    data: &mut TypeData,
    ast_data: AstData,
) -> Result<Expr, FrontendError> {
    let left = left.typecheck(data)?;
    let right = right.typecheck(data)?;
    if !((left.get_type() == right.get_type())
        || (op == Operator::Add
            && left.get_type().is_pointer()
            && right.get_type() == PrimType::Int.into()))
    {
        return Err(TypeError::BinaryTypeMissmatch(op, left.get_type(), right.get_type()).into());
    }

    if let TypeDef::Function(_) = left.get_type() {
        return Err(TypeError::BinaryOperatorError.into());
    }

    let t: TypeDef = match (op, left.get_type()) {
        (Operator::Add, TypeDef::PointerType(p)) => TypeDef::PointerType(p),
        (Operator::Sub, TypeDef::PointerType(p)) => TypeDef::PointerType(p),
        (Operator::Add, TypeDef::PrimType(t)) => t.into(),
        (Operator::Sub, TypeDef::PrimType(t)) => t.into(),
        (Operator::Mul, TypeDef::PrimType(t)) => t.into(),
        (Operator::Div, TypeDef::PrimType(t)) => t.into(),
        (
            Operator::Lt
            | Operator::Le
            | Operator::Gt
            | Operator::Ge
            | Operator::Eql
            | Operator::Neq,
            _,
        ) => PrimType::Int.into(),
        (Operator::Assign, _) => assign(&left, &right)?,
        (Operator::BitOr, TypeDef::PrimType(t)) => t.into(),
        (Operator::BitAnd, TypeDef::PrimType(t)) => t.into(),
        (Operator::And, TypeDef::PrimType(PrimType::Int)) => PrimType::Int.into(),
        (Operator::Or, TypeDef::PrimType(PrimType::Int)) => PrimType::Int.into(),
        (Operator::BitNot, TypeDef::PrimType(t)) => t.into(),
        (Operator::Mod, TypeDef::PrimType(PrimType::Int)) => PrimType::Int.into(),
        (Operator::ShiftLeft, TypeDef::PrimType(t)) => t.into(),
        (Operator::ShiftRight, TypeDef::PrimType(t)) => t.into(),
        _ => return Err(TypeError::BinaryOperatorError.into()),
    };

    let res = ExprType::BinOp(op, Box::new(left), Box::new(right));
    let res = Expr::new(res, ast_data).typed(t);
    Ok(res)
}

impl TypecheckAst<Expr> for Expr {
    fn typecheck(&self, data: &mut TypeData) -> Result<Expr, FrontendError> {
        match &self.value {
            ExprType::BinOp(op, left, right) => binary_op(
                op.clone(),
                left.clone(),
                right.clone(),
                data,
                self.data.clone(),
            ),
            ExprType::UnaryPreOp(op, e) => {
                let (e, t) = unary_op(op.clone(), e.clone(), data)?;
                let res = ExprType::UnaryPreOp(op.clone(), Box::new(e));
                let res = Expr::new(res, self.data.clone()).typed(t);
                Ok(res)
            }
            ExprType::UnaryPostOp(op, e) => {
                let (e, t) = unary_op(op.clone(), e.clone(), data)?;
                let res = ExprType::UnaryPostOp(op.clone(), Box::new(e));
                let res = Expr::new(res, self.data.clone()).typed(t);
                Ok(res)
            }
            ExprType::Value(v) => match v {
                Val::Integer(_) => Ok(self.typed(PrimType::Int.into())),
                Val::Char(_) => Ok(self.typed(PrimType::Char.into())),
            },
            ExprType::Ident(ident) => {
                let t = data.get_ident_type(ident)?;
                Ok(self.typed(t))
            }
            ExprType::Call(func, params) => {
                let func = func.typecheck(data)?;
                let mut typed_par: Vec<Expr> = vec![];

                let fn_type = if let TypeDef::Function(fn_type) = func.get_type() {
                    Ok::<FnType, FrontendError>(fn_type)
                } else {
                    Err(TypeError::NonFunctionCall.into())
                }?;

                if fn_type.params.len() != params.len() {
                    return Err(TypeError::WrongNumberOfParametes(
                        fn_type.params.len(),
                        params.len(),
                    )
                    .into());
                }

                for i in 0..params.len() {
                    let tmp = params[i].typecheck(data)?;
                    if tmp.get_type() != fn_type.params[i] {
                        return Err(TypeError::WrongParamType(
                            fn_type.params[i].clone(),
                            tmp.get_type(),
                        )
                        .into());
                    }
                    typed_par.push(tmp);
                }

                let res = ExprType::Call(Box::new(func), typed_par);
                let res = Expr::new(res, self.data.clone()).typed(*fn_type.ret_type);

                Ok(res)
            }
            ExprType::Index(object, index) => {
                let object = object.typecheck(data)?;
                let index = index.typecheck(data)?;
                if index.get_type() != PrimType::Int.into() {
                    return Err(TypeError::IndexMustBeInteger.into());
                }
                let t = if let TypeDef::PointerType(t) = object.get_type() {
                    Ok::<TypeDef, FrontendError>(*t)
                } else if let TypeDef::Array(arr) = object.get_type() {
                    Ok::<TypeDef, FrontendError>(*arr.inner_type)
                } else {
                    Err(TypeError::NonPointerDeref.into())
                }?;
                let res = ExprType::Index(Box::new(object), Box::new(index));
                let res = Expr::new(res, self.data.clone()).typed(t);
                Ok(res)
            }
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
                let res = Expr::new(res, self.data.clone())
                    .typed(TypeDef::PointerType(Box::new(tmp.get_type())));
                Ok(res)
            }
            ExprType::Cast(t, _) => Ok(self.typed(t.clone())),
            ExprType::FieldAccess(e, field) => {
                let tmp = e.typecheck(data)?;
                if let TypeDef::Struct(s) = tmp.get_type() {
                }
                todo!()
            },
        }
    }
}

impl TypecheckAst<VarDecl> for VarDecl {
    fn typecheck(&self, data: &mut TypeData) -> Result<VarDecl, FrontendError> {
        let t = self.var_type.clone();
        let t = data.translate_type(t)?;

        if !t.sized() {
            return Err(TypeError::TypeIsNotSized.into());
        }

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
            StatementType::If(cond, then_body) => {
                let cond = cond.typecheck(data)?;
                if cond.get_type() != PrimType::Int.into() {
                    return Err(TypeError::ConditionMustBeInt.into());
                }
                data.push_env();
                let then_body = then_body.typecheck(data)?;
                data.pop_env();
                let res = StatementType::If(cond, Box::new(then_body));
                let res = Statement::new(res, self.data.clone()).typed(TypeDef::Void);
                Ok(res)
            }
            StatementType::IfElse(cond, then_body, else_body) => {
                let cond = cond.typecheck(data)?;
                if cond.get_type() != PrimType::Int.into() {
                    return Err(TypeError::ConditionMustBeInt.into());
                }
                data.push_env();
                let then_body = then_body.typecheck(data)?;
                data.pop_env();
                data.push_env();
                let else_body = else_body.typecheck(data)?;
                data.pop_env();
                let res = StatementType::IfElse(cond, Box::new(then_body), Box::new(else_body));
                let res = Statement::new(res, self.data.clone()).typed(TypeDef::Void);
                Ok(res)
            }
            StatementType::For(init, cond, update, body) => {
                let init = match init.clone().map(|x| x.typecheck(data)) {
                    Some(Err(e)) => return Err(e),
                    Some(Ok(init)) => Some(Box::new(init)),
                    None => None,
                };
                let cond = match cond.clone().map(|x| x.typecheck(data)) {
                    Some(Err(e)) => return Err(e),
                    Some(Ok(init)) if init.get_type() == PrimType::Int.into() => Some(init),
                    Some(Ok(_)) => return Err(TypeError::ConditionMustBeInt.into()),
                    None => None,
                };
                let update = match update.clone().map(|x| x.typecheck(data)) {
                    Some(Err(e)) => return Err(e),
                    Some(Ok(init)) => Some(init),
                    None => None,
                };
                let body = body.typecheck(data)?;
                let res = StatementType::For(init, cond, update, Box::new(body));
                let res = Statement::new(res, self.data.clone()).typed(TypeDef::Void);
                Ok(res)
            }
            StatementType::While(cond, body) => {
                let cond = cond.typecheck(data)?;
                if cond.get_type() != PrimType::Int.into() {
                    return Err(TypeError::ConditionMustBeInt.into());
                }
                data.push_env();
                let body = body.typecheck(data)?;
                data.pop_env();
                let res = StatementType::While(cond, Box::new(body));
                let res = Statement::new(res, self.data.clone()).typed(TypeDef::Void);
                Ok(res)
            }
            StatementType::Break | StatementType::Continue => Ok(self.typed(TypeDef::Void)),
            StatementType::Return(e) => match (e, data.ret()) {
                (None, None) => Ok(self.typed(TypeDef::Void)),
                (None, Some(_)) => Err(TypeError::ExpectingRet.into()),
                (Some(_), None) => Err(TypeError::UnexpectedRet.into()),
                (Some(res), Some(exp)) => {
                    let res_typed = res.typecheck(data)?;

                    if res_typed.data.node_type != Some(exp.clone()) {
                        return Err(TypeError::ReturnTypeError(res_typed.get_type(), exp).into());
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
        let f_ret = data.translate_type(self.header.ret_type.clone())?;

        if !f_ret.sized() {
            return Err(TypeError::TypeIsNotSized.into());
        }

        if let Ok(t) = data.get_ident_type(&self.value.header.name) {
            match t {
                TypeDef::Function(f_type)
                    if !f_type.body_def
                        && f_type.params.len() == self.header.params.len()
                        && *f_type.ret_type == f_ret =>
                {
                    for i in 0..f_type.params.len() {
                        if f_type.params[i] != self.header.params[i].1 {
                            return Err(TypeError::TypeParametrMissmatch.into());
                        }
                    }
                    ()
                }
                _ => {
                    return Err(
                        TypeError::IdentAlreadyExists(self.value.header.name.clone()).into(),
                    )
                }
            }
        }

        let mut header = self.header.typed(TypeDef::Void);
        header.ret_type = f_ret;
        let mut translated_params: Vec<(String, TypeDef)> = vec![];
        for (name, var_type) in header.params.iter() {
            translated_params.push((name.clone(), data.translate_type(var_type.clone())?));
        }
        header.params = translated_params;
        let t: FnType = header.clone().into();

        data.add_force(&self.value.header.name, t.into())?;

        let body = if let Some(body) = self.value.body.clone() {
            data.push_fn(header.ret_type.clone());
            for (name, var_type) in header.params.iter() {
                data.add_var(name, var_type.clone())?;
            }
            let body = body.typecheck(data)?;
            data.pop_env();
            Some(body)
        } else {
            None
        };

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

impl TypecheckAst<StructDef> for StructDef {
    fn typecheck(&self, data: &mut TypeData) -> Result<StructDef, FrontendError> {
        data.add_type(
            &self.name,
            TypeDef::Struct(StructDefType {
                name: self.name.clone(),
                fields: None,
            }),
        );
        let fields: Option<Vec<VarDecl>> = if let Some(fields) = self.fields.clone() {
            data.push_env();
            let mut res = vec![];
            for field in fields {
                res.push(field.typecheck(data)?);
            }
            data.pop_env();
            Some(res)
        } else {
            None
        };
        let res = StructDefType {
            name: self.name.clone(),
            fields,
        };
        data.add_type(&self.name, TypeDef::Struct(res.clone()));
        let res = StructDef::new(res, self.data.clone());
        Ok(res)
    }
}

impl TypecheckAst<TopLevel> for TopLevel {
    fn typecheck(&self, data: &mut TypeData) -> Result<TopLevel, FrontendError> {
        match self {
            TopLevel::Function(f) => Ok(TopLevel::Function(f.typecheck(data)?)),
            TopLevel::Var(v) => Ok(TopLevel::Var(v.typecheck(data)?)),
            TopLevel::Structure(s) => Ok(TopLevel::Structure(s.typecheck(data)?)),
        }
    }
}

pub fn type_program(program: Program) -> Result<Program, FrontendError> {
    let mut data = TypeData::default();
    let mut res: Program = Program::default();

    for item in program.items {
        res.items.push(item.typecheck(&mut data)?);
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    fn type_ok(input: &str) {
        let lex = Lexer::new("tmp".to_string(), input.chars().peekable());
        let mut parser = Parser::new(lex).unwrap();
        let res = parser.parse().unwrap();
        let typed = type_program(res);
        println!("{:?}", typed);
        assert!(typed.is_ok());
    }

    fn type_err(input: &str) {
        let lex = Lexer::new("tmp".to_string(), input.chars().peekable());
        let mut parser = Parser::new(lex).unwrap();
        let res = parser.parse().unwrap();
        let typed = type_program(res);
        //println!("{:?}", typed);
        assert!(typed.is_err());
    }

    #[test]
    fn basic_test_typedef() {
        type_ok("int main() {}");
        type_ok("int main() {return 1;}");
        type_ok("int main() {int x = 5;}");
        type_err("int main() {int x = 5; char x;}");
        type_ok("int main() {int x = 5; return x;}");
        type_err("char main() {int x = 5; return x;}");
        type_ok("int a; int main() {return a;}");
        type_err("int * a; int main() {return a;}");
        type_ok("int f() { return 1; } int main() {return f();}");
    }

    #[test]
    fn block_test_typedef() {
        type_ok("int main() {int x; {int y = x;}}");
        type_ok("int main() {int x; {int z = x;{int y = z; return z;}}}");
        type_err("int main() {char x; {char z = x;{char y = z; return z;}}}");
        type_err("int main() {{int y = x;} int x;}");
        type_err("int main() {{int y = 5;} return y;}");
    }

    #[test]
    fn flow_test_typedef() {
        type_ok("int main() { if (1) return 1;  }");
        type_ok("int main() { if (0) return 1;  }");
        type_ok("int main() { while (0) return 1;  }");
        type_ok("int main() { int x = 5; while (x) return 1;  }");
        type_err("int main() { char x = 5; while (x) return 1;  }");
        type_ok("int main() { if(1) { int x; } return 1;}");
        type_err("int main() { if(1) int x; return x;}");
        type_ok("int main() { if(1) int x; else int x;}");
        type_err("int main() { if(1) int x; else int x; return x;}");
        type_ok("int main() { for (int i = 5; i < 10; i++) return 1;}");
        type_ok(
            "
            int main() { 
                for (int i = 5; i < 10; i++) {
                    int x = 2;
                    if (i > 2)
                        return 1;
                    else 
                        x = i + x;
                }
                return 2;
            }",
        );
    }

    #[test]
    fn index_test_typedef() {
        type_ok("int main() { int * x; return x[0]; }");
        type_ok("char * f() {} char main() { return f()[1]; }");
        type_err("int f() {} int main() { return f()[0]; }");
        type_err("int main() { int * x; return x[x]; }");
        type_err("int main() { char * x; return x[x]; }");
        type_err("int main() { char * x; return x[0]; }");
    }

    #[test]
    fn cast_test_typedef() {
        type_ok("int main() { char * x; return cast<int>(x[0]); }");
        type_ok("char main() { char * x; return x[cast<int>(x)]; }");
        type_ok("int f() {} char main() { return cast<char>(f); }");
        type_err("int f() {} char main() { return f; }");
    }

    #[test]
    fn deref_test_typedef() {
        type_ok("int main() { int * x; return *x; }");
        type_ok("int main() { int a = 5; int * x = &a; return *x; }");
        type_ok("int main() { int a = 5; int * b = &a; int ** x = &b; return **x; }");
        type_err("int main() { int a = 5; int * b = &a; int ** x = &b; return ***x; }");
        type_err("int main() { int a = 5; int * b = &a; int ** x = &b; return *x; }");
    }

    #[test]
    fn assign_test_typedef() {
        type_ok("int main() {int a = 5; a = 3; return a;}");
        type_ok("int main() {int * a; *a = 3; return *a;}");
        type_err("int main() {int * a; a = 3; return *a;}");
        type_err("int f() {} int main() { f = 5; }");
        type_err("int f() {} int main() { *f = 5; }");
        type_err("int main() { 1 = 5; }");
        type_err("int main() { *1 = 5; }");
        type_err("int main() { **1 = 5; }");
        type_err("int main() {int a = 5; *a = 3; return a;}");
    }

    #[test]
    fn binaryop_test_typedef() {}

    #[test]
    fn function_test_typedef() {
        type_ok("int f(int x) { return x + 1; } int main() { return f(2); }");
        type_err("int f(char x) { return cast<int>(x); } int main() { return f(2); }");
        type_err("int f(int x) { return x + 1; } int main() { return f(); }");
        type_err("int f(int x) { return x + 1; } int main() { return f('a'); }");
        type_err("int f(int x) { return x + 1; } int main() { return f(1, 2); }");
    }

    #[test]
    fn recur_test_typedef() {
        type_ok(
            "
            int fib(int n) {
                if (n <= 0) {
                    return 0;
                }
                else if (n <= 1) {
                    return 1;
                }
                return fib(n - 1) + fib(n - 2);
            }

            int main() {
                int res = fib(10);
                return res;
            }
        ",
        );

        type_ok(
            "
            int odd(int n);

            int even(int n) {
                return odd(n - 1);
            }

            int odd(int n) {
                return even(n - 1);
            }
        ",
        );

        type_err(
            "
            int even(int n) {
                return odd(n - 1);
            }

            int odd(int n) {
                return even(n - 1);
            }
        ",
        );

        type_err(
            "
            int even(int n) {
                return odd(n - 1);
            }

            int odd(int n);

            int odd(int n) {
                return even(n - 1);
            }
        ",
        );

        type_err(
            "
            int f(int a);
            int f(int a, char b) {}
        ",
        );

        type_err(
            "
            int f(int a);
            int f() {}
        ",
        );
    }

    #[test]
    fn struct_test_typedef() {
        type_ok("struct A; int main() {}");
        type_ok("struct A {} A f() {}");
        type_err("struct A; A f() {}");
        type_ok("struct A {} A v;");
        type_err("struct A; A v;");
        type_err("struct A { A a; }");
        type_ok("struct A { A * a; }");
        type_ok("struct A; A* a;");
        type_err("struct Foo {} Foo main(Foo argc) { return ++argc; }");
        type_ok("struct Foo; struct Foo { int i; } void main(Foo x) {}");
        type_ok("struct A {} A f() { A a; return a; }");
        type_ok("struct A {} struct B { A a; } ");
        type_err("struct B; struct A { B b; } struct B { A a; } ");
        type_ok("struct B; struct A { B * b; } struct B { A a; } ");
        type_ok("struct A {} A f(A a) { return a; }");
        type_ok("struct A {} A g() { A a; return a; } void f() { A a = g(); }");
        type_ok("struct A {} A g() { A a; return a; } A f() { return g(); }");
        type_err("struct A {} struct B {} A g() { A a; return a; } B f() { return g(); }");

        type_ok("struct A { int a; } int main() {A a; return a.a;}");
        //type_err("struct A { int a; } int main() {A a; return a.b;}");
        //type_err("struct A { int a; } int main() {A a; return a.a.a;}");
        //type_ok("struct A { int a; } int main() {A a; a.a = 5; return a.a;}");
        //type_ok("struct A { int a; } A f() {A a; return a;} int main() {f().a = 5; return a.a;}");
    }

    #[test]
    fn array_test_typedef() {
        type_ok("int main() {int * a; return a[0]; }");
        type_ok("int main() {int a[5]; return a[0]; }");
    }
}
