use std::{iter::Peekable, str::Chars};

use crate::errors::LexerError;

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Inc,
    Dec,
    Lt,
    Le,
    Gt,
    Ge,
    Eql,
    Neq,
    Assign,
    BitOr,
    Or,
    BitAnd,
    And,
    Not,
    BitNot,
}

impl Into<TokenType> for Operator {
    fn into(self) -> TokenType {
        TokenType::Operator(self)
    }
}

pub enum TokenType {
    Eof,
    Error,
    Ident(String),
    Operator(Operator),
    Int(i64),
    Double(f64),
}

pub struct Token {
    row: usize,
    col: usize,
    file_name: String,
    tok: TokenType,
}

pub struct Lexer<'a> {
    row: usize,
    col: usize,
    file_name: String,
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(file_name: String, input: Peekable<Chars<'a>>) -> Self {
        Self {
            row: 0,
            col: 0,
            file_name,
            input,
        }
    }

    fn create_token(&self, tok_type: TokenType) -> Token {
        Token {
            row: self.row,
            col: self.col,
            file_name: self.file_name.clone(),
            tok: tok_type,
        }
    }

    fn ignore_white(&mut self) -> Result<(), LexerError> {
        while self.peek_char()?.is_whitespace() {
            self.next_char()?;
        }
        Ok(())
    }

    fn peek_char(&mut self) -> Result<char, LexerError> {
        match self.input.peek() {
            Some(x) => Ok(x.clone()),
            None => Err(LexerError::UnexpectedEof),
        }
    }

    fn next_char(&mut self) -> Result<char, LexerError> {
        match self.input.next() {
            Some(x) => Ok(x.clone()),
            None => Err(LexerError::UnexpectedEof),
        }
    }

    fn eof(&mut self) -> bool {
        match self.input.peek() {
            Some(_) => false,
            None => true,
        }
    }

    pub fn get_token(&mut self) -> Result<Token, LexerError> {
        if self.eof() {
            return Ok(self.create_token(TokenType::Eof));
        }
        self.ignore_white()?;

        match self.peek_char()? {
            '+' => self.double_op('+', Operator::Add.into(), Operator::Inc.into()),
            '-' => self.double_op('-', Operator::Sub.into(), Operator::Dec.into()),
            '=' => self.double_op('=', Operator::Assign.into(), Operator::Eql.into()),
            '<' => self.double_op('=', Operator::Lt.into(), Operator::Le.into()),
            '>' => self.double_op('=', Operator::Gt.into(), Operator::Ge.into()),
            '|' => self.double_op('|', Operator::Or.into(), Operator::BitOr.into()),
            '&' => self.double_op('&', Operator::And.into(), Operator::BitAnd.into()),
            '!' => self.double_op('=', Operator::Not.into(), Operator::Neq.into()),
            '*' => Ok(self.create_token(Operator::Mul.into())),
            '/' => Ok(self.create_token(Operator::Mul.into())),
            '~' => Ok(self.create_token(Operator::BitNot.into())),
            c if c.is_alphabetic() || c == '_' => todo!(),
            c => Err(LexerError::UnexpectedCharacter(c)),
        }
    }

    fn double_op(
        &mut self,
        c: char,
        normal: TokenType,
        double: TokenType,
    ) -> Result<Token, LexerError> {
        let _tmp = self.next_char();
        if self.eof() {
            Ok(self.create_token(normal))
        } else if self.peek_char()? == c {
            Ok(self.create_token(double))
        } else {
            Ok(self.create_token(normal))
        }
    }

    pub fn ident(&mut self) -> Result<String, LexerError> {
        todo!()
    }

    pub fn num(&mut self) -> Result<i64, Lexer> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
