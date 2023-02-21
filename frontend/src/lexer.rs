use std::{iter::Peekable, str::Chars};

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
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
    data: TokenType,
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

}

#[cfg(test)]
mod tests {}
