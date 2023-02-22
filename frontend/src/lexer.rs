use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

use crate::errors::LexerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

trait IdentChar {
    fn is_ident_char(&self) -> bool;
}

impl IdentChar for char {
    fn is_ident_char(&self) -> bool {
        self.is_alphanumeric() || *self == '_'
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Char,
    If,
    Else,
    Void,
    While,
    For,
}

impl Into<TokenType> for Keyword {
    fn into(self) -> TokenType {
        TokenType::Kw(self)
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(Keyword::Int),
            "char" => Ok(Keyword::Char),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "void" => Ok(Keyword::Void),
            "while" => Ok(Keyword::While),
            "for" => Ok(Keyword::For),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Eof,
    Error,
    Ident(String),
    Operator(Operator),
    Int(i64),
    //Double(f64),
    Char(char),
    Kw(Keyword),
    LeftBrac,
    RightBrac,
    LeftCurly,
    RightCurly,
    Semicol,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn compare(&mut self, c : char) -> Result<(), LexerError> {
        if self.peek_char()? == c {
            let _ig = self.next_char();
            Ok(())
        }
        else {
            Err(LexerError::UnexpectedCharacter(self.peek_char()?))
        }
    }

    pub fn get_token(&mut self) -> Result<Token, LexerError> {
        if self.eof() {
            return Ok(self.create_token(TokenType::Eof));
        }
        self.ignore_white()?;
        let mut next = false;

        let mut single_char = |t: TokenType| {
            next = true;
            t
        };

        let res = match self.peek_char()? {
            '+' => self.double_op('+', Operator::Add.into(), Operator::Inc.into()),
            '=' => self.double_op('=', Operator::Assign.into(), Operator::Eql.into()),
            '<' => self.double_op('=', Operator::Lt.into(), Operator::Le.into()),
            '>' => self.double_op('=', Operator::Gt.into(), Operator::Ge.into()),
            '|' => self.double_op('|', Operator::Or.into(), Operator::BitOr.into()),
            '&' => self.double_op('&', Operator::And.into(), Operator::BitAnd.into()),
            '!' => self.double_op('=', Operator::Not.into(), Operator::Neq.into()),
            '*' => Ok(self.create_token(single_char(Operator::Mul.into()))),
            '/' => Ok(self.create_token(single_char(Operator::Div.into()))),
            '~' => Ok(self.create_token(single_char(Operator::BitNot.into()))),
            '(' => Ok(self.create_token(single_char(TokenType::LeftBrac))),
            ')' => Ok(self.create_token(single_char(TokenType::RightBrac))),
            '{' => Ok(self.create_token(single_char(TokenType::LeftCurly))),
            '}' => Ok(self.create_token(single_char(TokenType::RightCurly))),
            ';' => Ok(self.create_token(single_char(TokenType::Semicol))),
            c if c.is_alphabetic() || c == '_' => {
                let ident = self.ident()?;
                Ok(self.create_token(Self::check_keyword(ident)))
            }
            c if c.is_digit(10) && c != '0' => {
                let num = self.num()?;
                Ok(self.create_token(TokenType::Int(num)))
            }
            '-' => {
                let _ig = self.next_char();
                if self.eof() {
                    Ok(self.create_token(Operator::Sub.into()))
                } else if self.peek_char()? == '-' {
                    Ok(self.create_token(Operator::Dec.into()))
                } else if self.peek_char()?.is_digit(10) {
                    let num = self.num()?;
                    Ok(self.create_token(TokenType::Int(-num)))
                } else {
                    Ok(self.create_token(Operator::Sub.into()))
                }
            }
            '\'' => {
                let c = self.char_tok()?;
                Ok(self.create_token(TokenType::Char(c)))
            }
            c => Err(LexerError::UnexpectedCharacter(c)),
        }?;

        if next {
            let _ig = self.next_char();
        }

        Ok(res)
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

    fn ident(&mut self) -> Result<String, LexerError> {
        let mut res = "".to_string();
        while self.peek_char()?.is_ident_char() {
            res += self.peek_char()?.to_string().as_str();
            let _ig = self.next_char();
        }
        Ok(res)
    }

    pub fn num(&mut self) -> Result<i64, LexerError> {
        let mut res: i64 = 0;
        while self.peek_char()?.is_digit(10) {
            res *= 10;
            res += self.peek_char()?.to_digit(10).unwrap() as i64;
            let _ig = self.next_char();
        }
        Ok(res)
    }

    pub fn char_tok(&mut self) -> Result<char, LexerError> {
        self.compare('\'')?;
        let c = self.next_char()?;
        self.compare('\'')?;
        Ok(c)
    }

    pub fn check_keyword(ident: String) -> TokenType {
        match ident.parse::<Keyword>() {
            Ok(k) => k.into(),
            Err(_) => TokenType::Ident(ident),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test() {
        let input =
            "int main() {\nint x = 1 + 33; -1; 'a' if else while char(( _a -52 ))}".to_string();

        let mut lex = Lexer::new("filename.tc".to_string(), input.chars().peekable());
        
        let mut tokens : Vec<Token> = vec![];
        loop {
            let token = lex.get_token().unwrap();
            tokens.push(token);
            if tokens.last().unwrap().tok == TokenType::Eof {
                break;
            }
        }
        println!("{:?}", tokens.into_iter().map(|x| x.tok).collect::<Vec<TokenType>>());
        assert!(false);
    }
}
