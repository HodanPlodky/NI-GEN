use crate::{lexer::TokenType, ast::TypeDef};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LexerError {
    UnexpectedCharacter(char),
    UnexpectedEof,
    CharNotClosed,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParserError {
    Undefiened,
    UnexpectedToken(TokenType),
    InvalidType(TokenType),
    VarDeclInvalidName,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeError {
    VariableTypeError(String, TypeDef, TypeDef),
    ExpectingRet,
    UnexpectedRet,
    ReturnTypeError(TypeDef, TypeDef),
    IdentDoesNotExist(String),
    IdentAlreadyExists(String),
    NonPointerDeref,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FrontendError {
    Lexer(LexerError),
    Parser(ParserError),
    Type(TypeError),
}

impl From<LexerError> for FrontendError {
    fn from(e: LexerError) -> Self {
        FrontendError::Lexer(e)
    }
}

impl From<ParserError> for FrontendError {
    fn from(e: ParserError) -> Self {
        FrontendError::Parser(e)
    }
}

impl From<TypeError> for FrontendError {
    fn from(e: TypeError) -> Self {
        FrontendError::Type(e)
    }
}
