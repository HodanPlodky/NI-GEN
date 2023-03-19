use crate::{
    ast::{
        AstData, Expr, ExprType, FnDecl, FnDeclType, FnDef, FnDefType, PrimType, Program,
        Statement, StatementType, StructDef, StructDefType, TypeDef, Val, VarDecl, VarDeclType,
    },
    errors::{FrontendError, ParserError},
    lexer::{Keyword, Lexer, Operator, Token, TokenType},
};

pub struct Parser {
    lexer: Lexer,
    curr_tok: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Result<Self, FrontendError> {
        let mut lexer = lexer;
        let curr_tok = lexer.get_token()?;
        Ok(Self { lexer, curr_tok })
    }

    fn reset_to(&mut self, position: usize) -> Result<(), FrontendError> {
        self.lexer.reset_to(position);
        self.curr_tok = self.lexer.get_token()?;
        Ok(())
    }

    fn top(&self) -> Token {
        self.curr_tok.clone()
    }

    fn pop(&mut self) -> Token {
        let tmp = self.top();
        self.curr_tok = self.lexer.get_token().unwrap();
        tmp
    }

    fn compare(&mut self, tok: TokenType) -> Result<(), FrontendError> {
        let tmp = self.pop().tok;
        if tmp != tok {
            Err(ParserError::UnexpectedToken(tmp).into())
        } else {
            Ok(())
        }
    }

    fn act_data(&self) -> AstData {
        AstData::new(self.top().loc())
    }

    pub fn parse(&mut self) -> Result<Program, FrontendError> {
        let mut vars = vec![];
        let mut fns = vec![];
        let mut structs = vec![];

        while self.top().tok != TokenType::Eof {
            if self.top().tok == Keyword::Struct.into() {
                structs.push(self.struct_def()?);
            } else {
                let position = self.top().position;
                let try_fn = self.fn_decl();
                match try_fn {
                    Ok(fn_def) => fns.push(fn_def),
                    Err(_e) => {
                        self.reset_to(position)?;
                        vars.push(self.var_decl()?);
                        self.compare(TokenType::Semicol)?;
                    }
                };
            }
        }

        let main_fn = fns.iter().find(|x| x.header.name == "main").cloned();
        let fns = fns
            .into_iter()
            .filter(|x| x.header.name != "main")
            .collect();

        Ok(Program {
            var_decls: vars,
            structs,
            fn_defs: fns,
            fn_decl: vec![],
            main: main_fn,
        })
    }

    fn struct_def(&mut self) -> Result<StructDef, FrontendError> {
        let data = self.act_data();
        self.compare(Keyword::Struct.into())?;
        let name = self.get_ident()?;

        let fields = if self.top().tok == TokenType::Semicol {
            self.pop();
            None
        } else {
            self.compare(TokenType::LeftCurly)?;

            let mut vars: Vec<VarDecl> = vec![];
            while self.top().tok != TokenType::RightCurly {
                let var = self.var_decl()?;
                if let Some(_) = var.value.init_val {
                    return Err(ParserError::FieldCannotHaveInit.into());
                }
                vars.push(var);
                self.compare(TokenType::Semicol)?;
            }

            self.compare(TokenType::RightCurly)?;
            Some(vars)
        };

        let res = StructDefType { name, fields };
        let res = StructDef::new(res, data);

        Ok(res)
    }

    pub fn fn_decl(&mut self) -> Result<FnDef, FrontendError> {
        let data = self.act_data();
        let ret_type = self.type_parse()?;
        let tmp = self.pop().tok;
        let name = if let TokenType::Ident(x) = tmp {
            Ok(x)
        } else {
            Err(ParserError::UnexpectedToken(tmp))
        }?;

        self.compare(TokenType::LeftBrac)?;
        let mut params = vec![];

        if self.top().tok != TokenType::RightBrac {
            let t = self.type_parse()?;
            let i = self.get_ident()?;
            params.push((i, t));
            while self.top().tok == TokenType::Comma {
                self.pop();
                let t = self.type_parse()?;
                let i = self.get_ident()?;
                params.push((i, t));
            }
        }

        self.compare(TokenType::RightBrac)?;
        let body = if self.top().tok != TokenType::Semicol {
            Some(self.block_statement()?)
        } else {
            self.compare(TokenType::Semicol)?;
            None
        };

        let header = FnDeclType {
            name,
            params,
            ret_type,
        };

        let header = FnDecl::new(header, data.clone());

        Ok(FnDef::new(FnDefType { header, body }, data))
    }

    fn get_ident(&mut self) -> Result<String, FrontendError> {
        let tmp = self.pop().tok;
        if let TokenType::Ident(ident) = tmp {
            Ok(ident)
        } else {
            Err(ParserError::UnexpectedToken(tmp).into())
        }
    }

    fn statement(&mut self) -> Result<Statement, FrontendError> {
        let data = self.act_data();
        match self.top().tok {
            TokenType::Kw(Keyword::If) => self.if_statement(),
            TokenType::Kw(Keyword::While) => self.while_statement(),
            TokenType::Kw(Keyword::For) => self.for_statement(),
            TokenType::Kw(Keyword::Break) => {
                self.pop();
                self.compare(TokenType::Semicol)?;
                Ok(Statement::new(StatementType::Break, data))
            }
            TokenType::Kw(Keyword::Conti) => {
                self.pop();
                self.compare(TokenType::Semicol)?;
                Ok(Statement::new(StatementType::Continue, data))
            }
            TokenType::Kw(Keyword::Return) => {
                self.pop();
                let result = if self.top().tok != TokenType::Semicol {
                    StatementType::Return(Some(Box::new(self.expr()?)))
                } else {
                    StatementType::Return(None)
                };
                self.compare(TokenType::Semicol)?;
                Ok(Statement::new(result, data))
            }
            TokenType::LeftCurly => self.block_statement(),
            _ => {
                let res = self.expr_or_vars()?;
                self.compare(TokenType::Semicol)?;
                Ok(res)
            }
        }
    }

    fn if_statement(&mut self) -> Result<Statement, FrontendError> {
        let data = self.act_data();
        self.compare(Keyword::If.into())?;
        self.compare(TokenType::LeftBrac)?;
        let cond = self.expr()?;
        self.compare(TokenType::RightBrac)?;
        let then = self.statement()?;
        let value = if self.top().tok == Keyword::Else.into() {
            self.compare(Keyword::Else.into())?;
            let else_b = self.statement()?;
            StatementType::IfElse(cond, Box::new(then), Box::new(else_b))
        } else {
            StatementType::If(cond, Box::new(then))
        };
        Ok(Statement::new(value, data))
    }

    fn while_statement(&mut self) -> Result<Statement, FrontendError> {
        let data = self.act_data();
        self.compare(Keyword::While.into())?;

        self.compare(TokenType::LeftBrac)?;
        let cond = self.expr()?;
        self.compare(TokenType::RightBrac)?;

        let body = self.statement()?;
        Ok(Statement::new(
            StatementType::While(cond, Box::new(body)),
            data,
        ))
    }

    fn for_statement(&mut self) -> Result<Statement, FrontendError> {
        let data = self.act_data();
        self.compare(Keyword::For.into())?;
        self.compare(TokenType::LeftBrac)?;
        let var = if self.top().tok != TokenType::Semicol {
            Some(Box::new(self.expr_or_vars()?))
        } else {
            None
        };
        self.compare(TokenType::Semicol)?;

        let cond = if self.top().tok != TokenType::Semicol {
            Some(self.expr()?)
        } else {
            None
        };
        self.compare(TokenType::Semicol)?;

        let end = if self.top().tok != TokenType::RightBrac {
            Some(self.expr()?)
        } else {
            None
        };

        self.compare(TokenType::RightBrac)?;

        let body = self.statement()?;

        Ok(Statement::new(
            StatementType::For(var, cond, end, Box::new(body)),
            data,
        ))
    }

    fn block_statement(&mut self) -> Result<Statement, FrontendError> {
        let data = self.act_data();
        self.compare(TokenType::LeftCurly)?;
        let mut statements = vec![];

        while self.top().tok != TokenType::RightCurly {
            statements.push(self.statement()?);
        }
        self.compare(TokenType::RightCurly)?;

        Ok(Statement::new(StatementType::Block(statements), data))
    }

    fn var_decl(&mut self) -> Result<VarDecl, FrontendError> {
        let data = self.act_data();
        let var_type = self.type_parse()?;
        if let TokenType::Ident(name) = self.top().tok {
            self.pop();
            let init_val = if self.top().tok == Operator::Assign.into() {
                self.pop();
                Some(self.expr()?)
            } else {
                None
            };
            let result = VarDeclType {
                name,
                var_type,
                init_val,
            };
            Ok(VarDecl::new(result, data))
        } else {
            Err(ParserError::VarDeclInvalidName.into())
        }
    }

    fn expr_or_vars(&mut self) -> Result<Statement, FrontendError> {
        let p = self.top().position;
        let t = self.type_parse();
        self.reset_to(p)?;
        match t {
            Ok(_) => Ok(self.var_decl()?.into()),
            Err(_) => Ok(self.expr()?.into()),
        }
    }

    fn expr(&mut self) -> Result<Expr, FrontendError> {
        let mut e = self.e9()?;
        if self.top().tok == Operator::Assign.into() {
            let data = self.act_data();
            self.pop();
            e = Expr::new(
                ExprType::BinOp(Operator::Assign, Box::new(e), Box::new(self.expr()?)),
                data,
            );
        }
        Ok(e)
    }

    fn e9(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e8()?;
        while self.top().tok == Operator::Or.into() {
            let data = self.act_data();
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::new(
                    ExprType::BinOp(o, Box::new(result), Box::new(self.e8()?)),
                    data,
                );
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e8(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e7()?;
        while self.top().tok == Operator::And.into() {
            let data = self.act_data();
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::new(
                    ExprType::BinOp(o, Box::new(result), Box::new(self.e7()?)),
                    data,
                );
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e7(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e6()?;
        while self.top().tok == Operator::BitOr.into() {
            let data = self.act_data();
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::new(
                    ExprType::BinOp(o, Box::new(result), Box::new(self.e6()?)),
                    data,
                );
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e6(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e5()?;
        while self.top().tok == Operator::BitAnd.into() {
            let data = self.act_data();
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::new(
                    ExprType::BinOp(o, Box::new(result), Box::new(self.e5()?)),
                    data,
                );
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e5(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e4()?;
        while self.top().tok == Operator::Eql.into() || self.top().tok == Operator::Neq.into() {
            let data = self.act_data();
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::new(
                    ExprType::BinOp(o, Box::new(result), Box::new(self.e4()?)),
                    data,
                );
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e4(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e3()?;
        while self.top().tok == Operator::Lt.into()
            || self.top().tok == Operator::Le.into()
            || self.top().tok == Operator::Gt.into()
            || self.top().tok == Operator::Ge.into()
        {
            let data = self.act_data();
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::new(
                    ExprType::BinOp(o, Box::new(result), Box::new(self.e3()?)),
                    data,
                );
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e3(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e2()?;
        while self.top().tok == Operator::ShiftLeft.into()
            || self.top().tok == Operator::ShiftRight.into()
        {
            let data = self.act_data();
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::new(
                    ExprType::BinOp(o, Box::new(result), Box::new(self.e2()?)),
                    data,
                );
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e2(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e1()?;
        while self.top().tok == Operator::Add.into() || self.top().tok == Operator::Sub.into() {
            let data = self.act_data();
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::new(
                    ExprType::BinOp(o, Box::new(result), Box::new(self.e1()?)),
                    data,
                );
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e1(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e_unary_pre()?;
        while self.top().tok == Operator::Mul.into()
            || self.top().tok == Operator::Div.into()
            || self.top().tok == Operator::Mod.into()
        {
            let data = self.act_data();
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::new(
                    ExprType::BinOp(o, Box::new(result), Box::new(self.e7()?)),
                    data,
                );
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e_unary_pre(&mut self) -> Result<Expr, FrontendError> {
        let t = self.top().tok;
        if let TokenType::Operator(o) = t {
            self.pop();
            self.e_unary_pre_inner(o)
        } else {
            self.e_post()
        }
    }

    fn e_unary_pre_inner(&mut self, operator: Operator) -> Result<Expr, FrontendError> {
        let data = self.act_data();
        match operator {
            Operator::Add
            | Operator::Sub
            | Operator::Inc
            | Operator::Dec
            | Operator::Not
            | Operator::BitNot => Ok(Expr::new(
                ExprType::UnaryPreOp(operator, Box::new(self.e_unary_pre()?)),
                data,
            )),
            Operator::Mul => Ok(Expr::new(
                ExprType::Deref(Box::new(self.e_unary_pre()?)),
                data,
            )),
            Operator::BitAnd => Ok(Expr::new(
                ExprType::Address(Box::new(self.e_unary_pre()?)),
                data,
            )),
            _ => self.e_post(),
        }
    }

    fn e_post(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.f()?;

        loop {
            let data = self.act_data();
            match self.top().tok {
                TokenType::Operator(o) if o == Operator::Inc || o == Operator::Dec => {
                    self.pop();
                    result = Expr::new(ExprType::UnaryPostOp(o, Box::new(result)), data);
                }
                TokenType::LeftBrac => {
                    self.pop();
                    let mut args = vec![];
                    if self.top().tok != TokenType::RightBrac {
                        args.push(self.expr()?);
                        while self.top().tok == TokenType::Comma {
                            self.pop();
                            args.push(self.expr()?);
                        }
                    }
                    self.pop();
                    result = Expr::new(ExprType::Call(Box::new(result), args), data);
                }
                TokenType::LeftSquare => {
                    self.pop();
                    result = Expr::new(
                        ExprType::Index(Box::new(result), Box::new(self.expr()?)),
                        data,
                    );
                    self.compare(TokenType::RightSquare)?;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn f(&mut self) -> Result<Expr, FrontendError> {
        let data = self.act_data();
        match self.pop().tok {
            TokenType::Ident(name) => Ok(Expr::new(ExprType::Ident(name), data)),
            TokenType::Int(num) => Ok(Expr::new(ExprType::Value(Val::Integer(num)), data)),
            TokenType::Char(c) => Ok(Expr::new(ExprType::Value(Val::Char(c)), data)),
            TokenType::LeftBrac => {
                let e = self.expr()?;
                self.compare(TokenType::RightBrac)?;
                Ok(e)
            }
            TokenType::Kw(Keyword::Cast) => {
                self.compare(Operator::Lt.into())?;
                let t = self.type_parse()?;
                self.compare(Operator::Gt.into())?;
                self.compare(TokenType::LeftBrac)?;
                let e = self.expr()?;
                self.compare(TokenType::RightBrac)?;
                Ok(Expr::new(ExprType::Cast(t, Box::new(e)), data))
            }
            t => Err(ParserError::UnexpectedToken(t).into()),
        }
    }

    fn type_parse(&mut self) -> Result<TypeDef, FrontendError> {
        let mut t = match self.pop().tok {
            TokenType::Kw(Keyword::Void) => Ok(TypeDef::Void),
            TokenType::Kw(Keyword::Int) => Ok(TypeDef::PrimType(PrimType::Int)),
            TokenType::Kw(Keyword::Char) => Ok(TypeDef::PrimType(PrimType::Char)),
            t => Err(ParserError::InvalidType(t)),
        }?;

        while self.top().tok == Operator::Mul.into() {
            self.pop();
            t = TypeDef::PointerType(Box::new(t));
        }

        Ok(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program_ok(input: &str) {
        let lex = Lexer::new("tmp".to_string(), input.chars().peekable());
        let mut parser = Parser::new(lex).unwrap();
        let res = parser.parse();
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    fn program_err(input: &str) {
        let lex = Lexer::new("tmp".to_string(), input.chars().peekable());
        let mut parser = Parser::new(lex).unwrap();
        let res = parser.parse();
        println!("{:?}", res);
        assert!(res.is_err());
    }

    #[test]
    fn basic_program() {
        let input = "
            char f() {
                return 'a';
            }

            int main() {
                int a = 1 + 2;
                if (a > 0) {
                    a = 100;
                    return 1045;
                }
                else {
                    a = 1000;
                }
                return a;
            }
        ";
        program_ok(input);
        let input = "int main() {f();}";
        program_ok(input);
        let input = "int f() { return 1; } int main() {return f();}";
        program_ok(input);
        program_ok("int main() { for (int i = 5; i < 10; i++) return 1;}");
        program_ok("int main() { int * x = 3; return *x; }");
        program_ok("int f(int a, int b) {}");
    }

    #[test]
    fn struct_parse_test() {
        program_ok("
            struct A {
                int a;
            }
        ");

        program_ok("struct Structure; int main() {}");
        program_err("struct int;");
        program_ok("struct A{}");
        program_ok("struct A {int a; char b;}");
        program_err("struct A {int a = 1; char b;}");
    }

    #[test]
    fn wrong_program() {
        let input = "foo haha bubu";
        let lex = Lexer::new("tmp".to_string(), input.chars().peekable());
        let mut parser = Parser::new(lex).unwrap();
        let res = parser.parse();
        println!("{:?}", res);
        assert!(res.is_err());
    }
}
