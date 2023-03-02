use crate::{
    ast::{Expr, FnDecl, FnDef, PrimType, Program, Statement, TypeDef, Val, VarDecl},
    errors::{FrontendError, ParserError},
    lexer::{Keyword, Lexer, Operator, Token, TokenType},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_tok: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Result<Self, FrontendError> {
        let mut lexer = lexer;
        let curr_tok = lexer.get_token()?;
        Ok(Self { lexer, curr_tok })
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

    pub fn parse(&mut self) -> Result<Program, FrontendError> {
        let mut vars = vec![];
        let mut fns = vec![];

        while self.top().tok != TokenType::Eof {
            let position = self.lexer.position;
            let try_fn = self.fn_decl();
            match try_fn {
                Ok(fn_def) => fns.push(fn_def),
                Err(_) => {
                    self.lexer.reset_to(position);
                    vars.push(self.var_decl()?);
                }
            };
        }

        //let main_fn = fns.iter().find(|x| x.header.name == "main").copied();
        let main_fn = None;
        let fns = fns
            .into_iter()
            .filter(|x| x.header.name != "main")
            .collect();

        Ok(Program {
            var_decls: vars,
            fn_defs: fns,
            fn_decl: vec![],
            main: main_fn,
        })
    }

    pub fn fn_decl(&mut self) -> Result<FnDef, FrontendError> {
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
                let t = self.type_parse()?;
                let i = self.get_ident()?;
                params.push((i, t));
            }
        }

        self.compare(TokenType::RightBrac)?;
        let body = self.block_statement()?;

        let header = FnDecl {
            name,
            params,
            ret_type,
        };

        Ok(FnDef { header, body })
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
        match self.top().tok {
            TokenType::Kw(Keyword::If) => self.if_statement(),
            TokenType::Kw(Keyword::While) => self.while_statement(),
            TokenType::Kw(Keyword::For) => self.for_statement(),
            TokenType::Kw(Keyword::Break) => {
                self.pop();
                self.compare(TokenType::Semicol)?;
                Ok(Statement::Break)
            }
            TokenType::Kw(Keyword::Conti) => {
                self.pop();
                self.compare(TokenType::Semicol)?;
                Ok(Statement::Continue)
            }
            TokenType::Kw(Keyword::Return) => {
                self.pop();
                let result = if self.top().tok != TokenType::Semicol {
                    Statement::Return(Some(Box::new(self.expr()?)))
                } else {
                    Statement::Return(None)
                };
                self.compare(TokenType::Semicol)?;
                Ok(result)
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
        self.compare(Keyword::If.into())?;
        self.compare(TokenType::LeftBrac)?;
        let cond = self.expr()?;
        self.compare(TokenType::RightBrac)?;
        let then = self.statement()?;
        if self.top().tok == Keyword::Else.into() {
            self.compare(Keyword::Else.into())?;
            let else_b = self.statement()?;
            Ok(Statement::IfElse(cond, Box::new(then), Box::new(else_b)))
        } else {
            Ok(Statement::If(cond, Box::new(then)))
        }
    }

    fn while_statement(&mut self) -> Result<Statement, FrontendError> {
        self.compare(Keyword::While.into())?;

        self.compare(TokenType::LeftBrac)?;
        let cond = self.expr()?;
        self.compare(TokenType::RightBrac)?;

        let body = self.statement()?;
        Ok(Statement::While(cond, Box::new(body)))
    }

    fn for_statement(&mut self) -> Result<Statement, FrontendError> {
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

        let end = if self.top().tok != TokenType::Semicol {
            Some(self.expr()?)
        } else {
            None
        };
        self.compare(TokenType::RightBrac)?;

        let body = self.statement()?;

        Ok(Statement::For(var, cond, end))
    }

    fn block_statement(&mut self) -> Result<Statement, FrontendError> {
        self.compare(TokenType::LeftCurly)?;
        let mut statements = vec![];

        while self.top().tok != TokenType::RightBrac {
            statements.push(self.statement()?);
        }
        self.compare(TokenType::RightCurly)?;

        Ok(Statement::Block(statements))
    }

    fn var_decl(&mut self) -> Result<VarDecl, FrontendError> {
        let var_type = self.type_parse()?;
        if let TokenType::Ident(name) = self.top().tok {
            self.pop();
            let init_val = if self.top().tok == Operator::Assign.into() {
                self.pop();
                Some(self.expr()?)
            } else {
                None
            };
            let result = VarDecl {
                name,
                var_type,
                init_val,
            };
            Ok(result)
        } else {
            Err(ParserError::VarDeclInvalidName.into())
        }
    }

    fn expr_or_vars(&mut self) -> Result<Statement, FrontendError> {
        let p = self.lexer.position;
        let t = self.type_parse();
        self.lexer.reset_to(p);
        match t {
            Ok(_) => Ok(Statement::VarDecl(self.var_decl()?)),
            Err(_) => Ok(Statement::Expr(self.expr()?)),
        }
    }

    fn expr(&mut self) -> Result<Expr, FrontendError> {
        let mut e = self.e9()?;
        if self.top().tok == Operator::Assign.into() {
            self.pop();
            e = Expr::BinOp(Operator::Assign, Box::new(e), Box::new(self.expr()?));
        }
        Ok(e)
    }

    fn e9(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e8()?;
        while self.top().tok == Operator::Or.into() {
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::BinOp(o, Box::new(result), Box::new(self.e8()?))
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e8(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e7()?;
        while self.top().tok == Operator::And.into() {
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::BinOp(o, Box::new(result), Box::new(self.e7()?))
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e7(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e6()?;
        while self.top().tok == Operator::BitOr.into() {
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::BinOp(o, Box::new(result), Box::new(self.e6()?))
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e6(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e5()?;
        while self.top().tok == Operator::BitAnd.into() {
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::BinOp(o, Box::new(result), Box::new(self.e5()?))
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e5(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e4()?;
        while self.top().tok == Operator::Eql.into() || self.top().tok == Operator::Neq.into() {
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::BinOp(o, Box::new(result), Box::new(self.e4()?))
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
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::BinOp(o, Box::new(result), Box::new(self.e3()?))
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
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::BinOp(o, Box::new(result), Box::new(self.e2()?))
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e2(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e1()?;
        while self.top().tok == Operator::Add.into() || self.top().tok == Operator::Sub.into() {
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::BinOp(o, Box::new(result), Box::new(self.e1()?))
            } else {
                unreachable!();
            }
        }
        Ok(result)
    }

    fn e1(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.e7()?;
        while self.top().tok == Operator::Mul.into()
            || self.top().tok == Operator::Div.into()
            || self.top().tok == Operator::Mod.into()
        {
            if let TokenType::Operator(o) = self.pop().tok {
                result = Expr::BinOp(o, Box::new(result), Box::new(self.e7()?))
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
            self.e_unary_pre_inner(o);
        }
        todo!()
    }

    fn e_unary_pre_inner(&mut self, operator: Operator) -> Result<Expr, FrontendError> {
        match operator {
            Operator::Add
            | Operator::Sub
            | Operator::Inc
            | Operator::Dec
            | Operator::Not
            | Operator::BitNot => Ok(Expr::UnaryPreOp(operator, Box::new(self.e_unary_pre()?))),
            Operator::Mul => Ok(Expr::Deref(Box::new(self.e_unary_pre()?))),
            Operator::BitAnd => Ok(Expr::Address(Box::new(self.e_unary_pre()?))),
            _ => self.e_post(),
        }
    }

    fn e_post(&mut self) -> Result<Expr, FrontendError> {
        let mut result = self.f()?;

        loop {
            match self.top().tok {
                TokenType::Operator(o) if o == Operator::Inc || o == Operator::Dec => {
                    self.pop();
                    result = Expr::UnaryPostOp(o, Box::new(result));
                }
                TokenType::LeftBrac => {
                    let mut args = vec![];
                    if self.top().tok != TokenType::RightBrac {
                        args.push(self.expr()?);
                        while self.top().tok == TokenType::Comma {
                            self.pop();
                            args.push(self.expr()?);
                        }
                    }
                    result = Expr::Call(Box::new(result), args);
                }
                TokenType::LeftSquare => {
                    self.pop();
                    result = Expr::Index(Box::new(result), Box::new(self.expr()?));
                    self.compare(TokenType::RightSquare)?;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn f(&mut self) -> Result<Expr, FrontendError> {
        match self.pop().tok {
            TokenType::Ident(name) => Ok(Expr::Ident(name)),
            TokenType::Int(num) => Ok(Expr::Value(Val::Integer(num))),
            TokenType::Char(c) => Ok(Expr::Value(Val::Char(c))),
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
                Ok(Expr::Cast(t, Box::new(e)))
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
    #[test]
    fn test_expr() {}

    #[test]
    fn basic_program() {}
}
