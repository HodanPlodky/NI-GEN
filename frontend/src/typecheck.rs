use std::collections::HashMap;

use crate::{
    ast::{
        Expr, ExprType, FnDecl, FnDef, Program, Statement, StatementType, StructDef, StructDefType,
        TopLevel, Val, VarDecl,
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
    errors: Vec<TypeError>,
}

impl Default for TypeData {
    fn default() -> Self {
        Self {
            type_map: HashMap::new(),
            env: vec![EnvLevel::new(None)],
            errors: vec![],
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

    fn create_err(&mut self, err: TypeError) {
        self.errors.push(err);
    }

    fn throw(&self) -> Result<(), FrontendError> {
        if self.errors.len() > 0 {
            Err(FrontendError::Type(self.errors.clone()))
        } else {
            Ok(())
        }
    }
}

trait TypecheckAst<T>
where
    T: PartialEq + Eq + Clone,
{
    fn typecheck(&mut self, data: &mut TypeData) -> Result<TypeDef, FrontendError>;
}

fn unary_op(
    op: Operator,
    expr: &mut Box<Expr>,
    data: &mut TypeData,
) -> Result<TypeDef, FrontendError> {
    expr.typecheck(data)?;
    let t = match (op, expr.get_type()) {
        (Operator::Not, TypeDef::PrimType(PrimType::Int)) => PrimType::Int.into(),
        (_, TypeDef::PrimType(t)) => t.into(),
        (_, TypeDef::PointerType(t)) => TypeDef::PointerType(t),
        _ => return Err(TypeError::InvalidOperation(op).into()),
    };

    Ok(t)
}

impl Expr {
    fn assignable(&self) -> bool {
        match &self.value {
            ExprType::Ident(_) => {
                if let TypeDef::Function(_) = self.get_type() {
                    false
                } else {
                    true
                }
            }
            ExprType::Index(arr, _) => arr.assignable(),
            ExprType::Deref(_) => true,
            ExprType::FieldAccess(s, _) => s.assignable(),
            _ => false,
        }
    }
}

fn assign(left: &Expr, _right: &Expr) -> Result<TypeDef, FrontendError> {
    if left.assignable() {
        Ok(TypeDef::Void)
    } else {
        Err(TypeError::CannotAssignInto(left.clone()).into())
    }
}

fn binary_op(
    op: Operator,
    left: &mut Box<Expr>,
    right: &mut Box<Expr>,
    data: &mut TypeData,
) -> Result<TypeDef, FrontendError> {
    left.typecheck(data)?;
    right.typecheck(data)?;
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

    Ok(t)
}

impl TypecheckAst<Expr> for Expr {
    fn typecheck(&mut self, data: &mut TypeData) -> Result<TypeDef, FrontendError> {
        match &mut self.value {
            ExprType::BinOp(op, left, right) => {
                let t = binary_op(op.clone(), left, right, data)?;
                self.set_type(t);
                Ok(TypeDef::Void)
            }

            ExprType::UnaryPreOp(op, e) => {
                let t = unary_op(op.clone(), e, data)?;
                self.set_type(t);
                Ok(TypeDef::Void)
            }

            ExprType::UnaryPostOp(op, e) => {
                let t = unary_op(op.clone(), e, data)?;
                self.set_type(t);
                Ok(TypeDef::Void)
            }

            ExprType::Value(v) => match v {
                Val::Integer(_) => {
                    self.set_type(PrimType::Int.into());
                    Ok(TypeDef::Void)
                }
                Val::Char(_) => {
                    self.set_type(PrimType::Char.into());
                    Ok(TypeDef::Void)
                }
            },

            ExprType::Ident(ident) => {
                let t = data.get_ident_type(ident)?;
                self.set_type(t);
                Ok(TypeDef::Void)
            }

            ExprType::Call(func, params) => {
                func.typecheck(data)?;

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
                    params[i].typecheck(data)?;
                    if params[i].get_type() != fn_type.params[i] {
                        return Err(TypeError::WrongParamType(
                            fn_type.params[i].clone(),
                            params[i].get_type(),
                        )
                        .into());
                    }
                }
                self.set_type(*fn_type.ret_type);

                Ok(TypeDef::Void)
            }

            ExprType::Index(object, index) => {
                object.typecheck(data)?;
                index.typecheck(data)?;
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
                self.set_type(t);
                Ok(TypeDef::Void)
            }

            ExprType::Deref(e) => {
                e.typecheck(data)?;
                let t = if let TypeDef::PointerType(t) = e.get_type() {
                    Ok::<Box<TypeDef>, FrontendError>(t)
                } else {
                    Err(TypeError::NonPointerDeref.into())
                }?;
                self.set_type(*t);
                Ok(TypeDef::Void)
            }
            ExprType::Address(e) => {
                e.typecheck(data)?;
                let t = e.get_type();
                self.set_type(TypeDef::PointerType(Box::new(t)));
                Ok(TypeDef::Void)
            }
            ExprType::Cast(t, _) => {
                let t = t.clone();
                self.set_type(t);
                Ok(TypeDef::Void)
            }
            ExprType::FieldAccess(e, field) => {
                e.typecheck(data)?;
                if let TypeDef::Struct(s) = e.get_type() {
                    if let Some(t) = s.field_type(field) {
                        self.set_type(t);
                        Ok(TypeDef::Void)
                    } else {
                        Err(TypeError::MissingField(field.clone()).into())
                    }
                } else {
                    Err(TypeError::NonStructType.into())
                }
            }
        }
    }
}

impl TypecheckAst<VarDecl> for VarDecl {
    fn typecheck(&mut self, data: &mut TypeData) -> Result<TypeDef, FrontendError> {
        let t = self.var_type.clone();
        let t = data.translate_type(t)?;

        if !t.sized() {
            return Err(TypeError::TypeIsNotSized.into());
        }

        data.add_var(&self.name, t.clone())?;
        let name = self.value.name.clone();
        if let Some(init) = &mut self.init_val {
            init.typecheck(data)?;

            if init.get_type() != t {
                return Err(TypeError::VariableTypeError(name, t, init.get_type()).into());
            }

            self.set_type(TypeDef::Void);
        } else {
            self.set_type(TypeDef::Void);
        }
        Ok(TypeDef::Void)
    }
}

impl TypecheckAst<Statement> for Statement {
    fn typecheck(&mut self, data: &mut TypeData) -> Result<TypeDef, FrontendError> {
        match &mut self.value {
            StatementType::Expr(e) => {
                e.typecheck(data)?;
                let t = e.get_type();
                self.set_type(t);
                Ok(TypeDef::Void)
            }
            StatementType::VarDecl(v) => {
                v.typecheck(data)?;
                let t = v.get_type();
                self.set_type(t);
                Ok(TypeDef::Void)
            }
            StatementType::Block(stmts) => {
                let mut ret_type = TypeDef::Void;
                data.push_env();
                for s in stmts {
                    ret_type = s.typecheck(data)?;
                }
                data.pop_env();
                self.set_type(TypeDef::Void);
                Ok(ret_type)
            }
            StatementType::If(cond, then_body) => {
                cond.typecheck(data)?;
                if cond.get_type() != PrimType::Int.into() {
                    return Err(TypeError::ConditionMustBeInt.into());
                }
                data.push_env();
                then_body.typecheck(data)?;
                data.pop_env();
                self.set_type(TypeDef::Void);
                Ok(TypeDef::Void)
            }
            StatementType::IfElse(cond, then_body, else_body) => {
                cond.typecheck(data)?;
                if cond.get_type() != PrimType::Int.into() {
                    return Err(TypeError::ConditionMustBeInt.into());
                }
                data.push_env();
                let then_ret = then_body.typecheck(data)?;
                data.pop_env();
                data.push_env();
                let else_ret = else_body.typecheck(data)?;
                data.pop_env();
                self.set_type(TypeDef::Void);

                if then_ret == else_ret {
                    Ok(then_ret)
                } else {
                    Ok(TypeDef::Void)
                }
            }
            StatementType::For(init, cond, update, body) => {
                if let Some(init) = init {
                    init.typecheck(data)?;
                }
                if let Some(cond) = cond {
                    cond.typecheck(data)?;
                    if cond.get_type() != PrimType::Int.into() {
                        return Err(TypeError::ConditionMustBeInt.into());
                    }
                }
                if let Some(update) = update {
                    update.typecheck(data)?;
                }
                body.typecheck(data)?;
                self.set_type(TypeDef::Void);
                Ok(TypeDef::Void)
            }
            StatementType::While(cond, body) => {
                cond.typecheck(data)?;
                if cond.get_type() != PrimType::Int.into() {
                    return Err(TypeError::ConditionMustBeInt.into());
                }
                data.push_env();
                body.typecheck(data)?;
                data.pop_env();
                self.set_type(TypeDef::Void);
                Ok(TypeDef::Void)
            }
            StatementType::Break | StatementType::Continue => {
                self.set_type(TypeDef::Void);
                Ok(TypeDef::Void)
            }
            StatementType::Return(e) => match (e, data.ret()) {
                (None, None) => {
                    self.set_type(TypeDef::Void);
                    Ok(TypeDef::Void)
                }
                (None, Some(TypeDef::Void)) => {
                    self.set_type(TypeDef::Void);
                    Ok(TypeDef::Void)
                }
                (None, Some(_)) => Err(TypeError::ExpectingRet.into()),
                (Some(_), None) => Err(TypeError::UnexpectedRet.into()),
                (Some(res), Some(exp)) => {
                    res.typecheck(data)?;

                    if res.data.node_type != Some(exp.clone()) {
                        return Err(TypeError::ReturnTypeError(res.get_type(), exp).into());
                    }

                    let ret_type = res.get_type().clone();
                    self.set_type(TypeDef::Void);
                    Ok(ret_type)
                }
            },
        }
    }
}

impl TypecheckAst<FnDef> for FnDef {
    fn typecheck(&mut self, data: &mut TypeData) -> Result<TypeDef, FrontendError> {
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

        self.header.typed(TypeDef::Void);
        self.header.ret_type = f_ret;
        let mut translated_params: Vec<(String, TypeDef)> = vec![];
        for (name, var_type) in self.header.params.iter() {
            translated_params.push((name.clone(), data.translate_type(var_type.clone())?));
        }
        self.header.params = translated_params;
        let t: FnType = self.header.clone().into();

        data.add_force(&self.value.header.name, t.into())?;

        let ret_type = self.header.ret_type.clone();
        let params = self.header.params.clone();
        if let Some(body) = &mut self.value.body {
            data.push_fn(ret_type);
            for (name, var_type) in params {
                data.add_var(&name, var_type.clone())?;
            }
            let ret_type = body.typecheck(data)?;
            data.pop_env();

            if ret_type != self.header.ret_type {
                return Err(TypeError::ExpectingRet.into());
            }
        }

        self.set_type(TypeDef::Void);
        Ok(TypeDef::Void)
    }
}

impl TypecheckAst<FnDecl> for FnDecl {
    fn typecheck(&mut self, data: &mut TypeData) -> Result<TypeDef, FrontendError> {
        let t: FnType = self.clone().into();
        data.add_var(&self.name, t.into())?;
        self.set_type(TypeDef::Void);
        Ok(TypeDef::Void)
    }
}

impl TypecheckAst<StructDef> for StructDef {
    fn typecheck(&mut self, data: &mut TypeData) -> Result<TypeDef, FrontendError> {
        data.add_type(
            &self.name,
            TypeDef::Struct(StructDefType {
                name: self.name.clone(),
                fields: None,
            }),
        );
        if let Some(fields) = &mut self.fields {
            data.push_env();
            for field in fields {
                field.typecheck(data)?;
            }
            data.pop_env();
        }
        data.add_type(&self.name, TypeDef::Struct(self.value.clone()));
        Ok(TypeDef::Void)
    }
}

impl TypecheckAst<TopLevel> for TopLevel {
    fn typecheck(&mut self, data: &mut TypeData) -> Result<TypeDef, FrontendError> {
        match self {
            TopLevel::Function(f) => f.typecheck(data)?,
            TopLevel::Var(v) => v.typecheck(data)?,
            TopLevel::Structure(s) => s.typecheck(data)?,
        };
        Ok(TypeDef::Void)
    }
}

pub fn type_program(program: &mut Program) -> Result<(), FrontendError> {
    let mut data = TypeData::default();

    for i in 0..program.items.len() {
        program.items[i].typecheck(&mut data)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    fn type_ok(input: &str) {
        let lex = Lexer::new("tmp".to_string(), input.chars().peekable());
        let mut parser = Parser::new(lex).unwrap();
        let mut res = parser.parse().unwrap();
        let tmp = type_program(&mut res);
        println!("{:?}", tmp);
        assert!(tmp.is_ok());
    }

    fn type_err(input: &str) {
        let lex = Lexer::new("tmp".to_string(), input.chars().peekable());
        let mut parser = Parser::new(lex).unwrap();
        let mut res = parser.parse().unwrap();
        let tmp = type_program(&mut res);
        println!("{:?}", res);
        assert!(tmp.is_err());
    }

    #[test]
    fn basic_test_typedef() {
        type_ok("void main() {}");
        type_ok("int main() {return 1;}");
        type_ok("void main() {int x = 5;}");
        type_err("void main() {int x = 5; char x;}");
        type_ok("int main() {int x = 5; return x;}");
        type_err("char main() {int x = 5; return x;}");
        type_ok("int a; int main() {return a;}");
        type_err("int * a; int main() {return a;}");
        type_ok("int f() { return 1; } int main() {return f();}");
    }

    #[test]
    fn block_test_typedef() {
        type_ok("void main() {int x; {int y = x;}}");
        type_ok("int main() {int x; {int z = x;{int y = z; return z;}}}");
        type_err("int main() {char x; {char z = x;{char y = z; return z;}}}");
        type_err("void main() {{int y = x;} int x;}");
        type_err("int main() {{int y = 5;} return y;}");
    }

    #[test]
    fn flow_test_typedef() {
        type_ok("int main() { if (1) return 1; return 2; }");
        type_ok("int main() { if (0) return 1; return 2; }");
        type_ok("int main() { while (0) return 1; return 2; }");
        type_ok("int main() { int x = 5; while (x) return 1; return 2; }");
        type_err("int main() { char x = 5; while (x) return 1; return 2; }");
        type_ok("int main() { if(1) { int x; } return 1;}");
        type_err("int main() { if(1) int x; return x;}");
        type_ok("void main() { if(1) int x; else int x;}");
        type_err("int main() { if(1) int x; else int x; return x;}");
        type_ok("int main() { for (int i = 5; i < 10; i++) return 1; return 2;}");
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
        type_ok("char * f(); char main() { return f()[1]; }");
        type_err("int f(); int main() { return f()[0]; }");
        type_err("int main() { int * x; return x[x]; }");
        type_err("int main() { char * x; return x[x]; }");
        type_err("int main() { char * x; return x[0]; }");
    }

    #[test]
    fn cast_test_typedef() {
        type_ok("int main() { char * x; return cast<int>(x[0]); }");
        type_ok("char main() { char * x; return x[cast<int>(x)]; }");
        type_ok("void f() {} char main() { return cast<char>(f); }");
        type_err("void f() {} char main() { return f; }");
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
        type_ok("struct A; void main() {}");
        type_ok("struct A {} A f() {A a; return a;}");
        type_err("struct A; A f();");
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
        type_err("struct A { int a; } int main() {A a; return a.b;}");
        type_err("struct A { int a; } int main() {A a; return a.a.a;}");
        type_ok("struct A { int a; } int main() {A a; a.a = 5; return a.a;}");
        type_err("struct A { int a; } A f() {A a; return a;} int main() {f().a = 5; return 1;}");
    }

    #[test]
    fn array_test_typedef() {
        type_ok("int main() {int * a; return a[0]; }");
        type_ok("int main() {int a[5]; return a[0]; }");
        type_ok("int main() {int a[5]; a[0] = 5; return a[0];}");
        type_err("int * f() {int a[5]; return a;} int main() {f()[0] = 5; return 1;}");
    }

    #[test]
    fn return_test_typedef() {
        type_ok("void main() { return; }");
        type_ok("int main(int a) { return a; }");
        type_err("int main(int a) { if (a) {return a;}  }");
        type_ok("int main(int a) { if (a) {return a;} return 1;}");
        type_ok("int main(int a) { if (a) {return a;} else {return 1;} }");
        type_err("int main(int a) { while (a) {return a;}  }");
        type_ok("int main(int a) { while (a) {return a;} return 1;}");
    }
}
