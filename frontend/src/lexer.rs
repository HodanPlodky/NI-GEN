use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

use crate::{ast::Operator, errors::LexerError};

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
    Cast,
    Break,
    Conti,
    Return,
    Struct,
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
            "cast" => Ok(Keyword::Cast),
            "break" => Ok(Keyword::Break),
            "continue" => Ok(Keyword::Conti),
            "return" => Ok(Keyword::Return),
            "struct" => Ok(Keyword::Struct),
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
    LeftSquare,
    RightSquare,
    Semicol,
    Comma,
    Dot,
    At,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub position: Loc,
    file_name: String,
    pub tok: TokenType,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Loc {
    row: usize,
    col: usize,
    position: usize,
}

pub struct Lexer {
    act_loc: Loc,
    last_loc: Loc,
    file_name: String,
    input: Vec<char>,
}

impl Lexer {
    pub fn new(file_name: String, input: Peekable<Chars<'_>>) -> Self {
        Self {
            act_loc: Loc::default(),
            last_loc: Loc::default(),
            file_name,
            input: input.collect(),
        }
    }

    pub fn reset_to(&mut self, position: Loc) {
        self.act_loc = position;
        self.last_loc = position;
    }

    fn create_token(&self, tok_type: TokenType) -> Token {
        Token {
            position: self.last_loc,
            file_name: self.file_name.clone(),
            tok: tok_type,
        }
    }

    fn ignore_white(&mut self) -> Result<(), LexerError> {
        while let Ok(x) = self.peek_char() {
            if !x.is_whitespace() {
                break;
            }
            self.next_char()?;
        }
        Ok(())
    }

    fn peek_char(&mut self) -> Result<char, LexerError> {
        if self.act_loc.position < self.input.len() {
            Ok(self.input[self.act_loc.position].clone())
        } else {
            Err(LexerError::UnexpectedEof)
        }
    }

    fn next_char(&mut self) -> Result<char, LexerError> {
        if self.act_loc.position < self.input.len() {
            self.act_loc.col += 1;
            if self.input[self.act_loc.position] == '\n' {
                self.act_loc.col = 0;
                self.act_loc.row += 1;
            }
            self.act_loc.position += 1;
            Ok(self.input[self.act_loc.position - 1].clone())
        } else {
            Err(LexerError::UnexpectedEof)
        }
    }

    fn eof(&mut self) -> bool {
        self.act_loc.position >= self.input.len()
    }

    fn compare(&mut self, c: char) -> Result<(), LexerError> {
        if self.peek_char()? == c {
            let _ig = self.next_char();
            Ok(())
        } else {
            Err(LexerError::UnexpectedCharacter(self.peek_char()?))
        }
    }

    pub fn get_token(&mut self) -> Result<Token, LexerError> {
        self.ignore_white()?;
        self.last_loc = self.act_loc.clone();
        if self.eof() {
            return Ok(self.create_token(TokenType::Eof));
        }
        let mut next = false;

        let mut single_char = |t: TokenType| {
            next = true;
            t
        };

        self.last_loc = self.act_loc.clone();

        let res = match self.peek_char()? {
            '+' => self.double_op('+', Operator::Add.into(), Operator::Inc.into()),
            '-' => self.double_op('-', Operator::Sub.into(), Operator::Dec.into()),
            '=' => self.double_op('=', Operator::Assign.into(), Operator::Eql.into()),
            '<' => self.double_op('=', Operator::Lt.into(), Operator::Le.into()),
            '>' => self.double_op('=', Operator::Gt.into(), Operator::Ge.into()),
            '|' => self.double_op('|', Operator::BitOr.into(), Operator::Or.into()),
            '&' => self.double_op('&', Operator::BitAnd.into(), Operator::And.into()),
            '!' => self.double_op('=', Operator::Not.into(), Operator::Neq.into()),
            '*' => Ok(self.create_token(single_char(Operator::Mul.into()))),
            '/' => Ok(self.create_token(single_char(Operator::Div.into()))),
            '%' => Ok(self.create_token(single_char(Operator::Mod.into()))),
            '~' => Ok(self.create_token(single_char(Operator::BitNot.into()))),
            '(' => Ok(self.create_token(single_char(TokenType::LeftBrac))),
            ')' => Ok(self.create_token(single_char(TokenType::RightBrac))),
            '{' => Ok(self.create_token(single_char(TokenType::LeftCurly))),
            '}' => Ok(self.create_token(single_char(TokenType::RightCurly))),
            '[' => Ok(self.create_token(single_char(TokenType::LeftSquare))),
            ']' => Ok(self.create_token(single_char(TokenType::RightSquare))),
            ';' => Ok(self.create_token(single_char(TokenType::Semicol))),
            ',' => Ok(self.create_token(single_char(TokenType::Comma))),
            '0' => Ok(self.create_token(single_char(TokenType::Int(0)))),
            '.' => Ok(self.create_token(single_char(TokenType::Dot))),
            '@' => Ok(self.create_token(single_char(TokenType::At))),
            c if c.is_alphabetic() || c == '_' => {
                let ident = self.ident()?;
                Ok(self.create_token(Self::check_keyword(ident)))
            }
            c if c.is_digit(10) && c != '0' => {
                let num = self.num()?;
                Ok(self.create_token(TokenType::Int(num)))
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
            let _tmp = self.next_char();
            Ok(self.create_token(double))
        } else {
            Ok(self.create_token(normal))
        }
    }

    fn ident(&mut self) -> Result<String, LexerError> {
        let mut res = "".to_string();
        while let Ok(x) = self.peek_char() {
            if !x.is_ident_char() {
                break;
            }
            res += self.peek_char()?.to_string().as_str();
            match self.next_char() {
                Ok(_) => (),
                Err(_) => break,
            };
        }
        Ok(res)
    }

    pub fn num(&mut self) -> Result<i64, LexerError> {
        let mut res: i64 = 0;
        while let Ok(x) = self.peek_char() {
            if !x.is_digit(10) {
                break;
            }
            res *= 10;
            res += self.peek_char()?.to_digit(10).unwrap() as i64;
            match self.next_char() {
                Ok(_) => (),
                Err(_) => {
                    break;
                }
            };
        }
        Ok(res)
    }

    pub fn char_tok(&mut self) -> Result<char, LexerError> {
        self.compare('\'')?;
        let c = if self.peek_char()? == '\\' {
            self.compare('\\')?;
            match self.next_char()? {
                'n' => '\n',
                x => return Err(LexerError::UnexpectedCharacter(x)),
            }
        } else {
            self.next_char()?
        };
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
            "int main() {\nint x = 1 + 33; x += 1 ;-1; 'a' if else while char(( _a -52 ))} 52++"
                .to_string();

        let mut lex = Lexer::new("filename.tc".to_string(), input.chars().peekable());

        let mut tokens: Vec<Token> = vec![];
        loop {
            let token = lex.get_token().unwrap();
            tokens.push(token);
            if tokens.last().unwrap().tok == TokenType::Eof {
                break;
            }
        }
        println!(
            "{:?}",
            tokens
                .iter()
                .map(|x| x.clone().tok)
                .collect::<Vec<TokenType>>()
        );

        let correct: Vec<TokenType> = vec![
            Keyword::Int.into(),
            TokenType::Ident("main".to_string()),
            TokenType::LeftBrac,
            TokenType::RightBrac,
            TokenType::LeftCurly,
            Keyword::Int.into(),
            TokenType::Ident("x".to_string()),
            Operator::Assign.into(),
            TokenType::Int(1),
            Operator::Add.into(),
            TokenType::Int(33),
            TokenType::Semicol,
            TokenType::Ident("x".to_string()),
            Operator::Add.into(),
            Operator::Assign.into(),
            TokenType::Int(1),
            TokenType::Semicol,
            Operator::Sub.into(),
            TokenType::Int(1),
            TokenType::Semicol,
            TokenType::Char('a'),
            Keyword::If.into(),
            Keyword::Else.into(),
            Keyword::While.into(),
            Keyword::Char.into(),
            TokenType::LeftBrac,
            TokenType::LeftBrac,
            TokenType::Ident("_a".to_string()),
            Operator::Sub.into(),
            TokenType::Int(52),
            TokenType::RightBrac,
            TokenType::RightBrac,
            TokenType::RightCurly,
            TokenType::Int(52),
            Operator::Inc.into(),
            TokenType::Eof,
        ];
        assert_eq!(
            tokens
                .into_iter()
                .map(|x| x.tok)
                .collect::<Vec<TokenType>>(),
            correct
        );
    }

    #[test]
    fn tmp() {
        let input = "52".to_string();

        let mut lex = Lexer::new("filename.tc".to_string(), input.chars().peekable());
        let mut tokens: Vec<Token> = vec![];
        loop {
            let token = lex.get_token().unwrap();
            tokens.push(token);
            if tokens.last().unwrap().tok == TokenType::Eof {
                break;
            }
        }
        println!(
            "{:?}",
            tokens
                .into_iter()
                .map(|x| x.tok)
                .collect::<Vec<TokenType>>()
        );
    }

    #[test]
    fn test_reset() {
        let input = "52".to_string();

        let mut lex = Lexer::new("filename.tc".to_string(), input.chars().peekable());
        let mut tokens: Vec<Token> = vec![];
        loop {
            let token = lex.get_token().unwrap();
            tokens.push(token);
            if tokens.last().unwrap().tok == TokenType::Eof {
                break;
            }
        }
        println!(
            "{:?}",
            tokens
                .into_iter()
                .map(|x| x.tok)
                .collect::<Vec<TokenType>>()
        );
    }

    #[test]
    fn test_add() {
        let input = "+=".to_string();

        let mut lex = Lexer::new("filename.tc".to_string(), input.chars().peekable());
        let mut tokens: Vec<Token> = vec![];
        loop {
            let token = lex.get_token().unwrap();
            tokens.push(token);
            if tokens.last().unwrap().tok == TokenType::Eof {
                break;
            }
        }

        let result: Vec<TokenType> = vec![
            Operator::Add.into(),
            Operator::Assign.into(),
            TokenType::Eof,
        ];

        assert_eq!(
            tokens
                .iter()
                .map(|x| x.tok.clone())
                .collect::<Vec<TokenType>>(),
            result
        );

        println!(
            "{:?}",
            tokens
                .into_iter()
                .map(|x| x.tok)
                .collect::<Vec<TokenType>>()
        );
    }
}
