use crate::{
    ast::Expr,
    lexer::{Operator, TokenType},
    typeast::TypeDef,
};

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
    FieldCannotHaveInit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeError {
    VariableTypeError(String, TypeDef, TypeDef),
    ExpectingRet,
    UnexpectedRet,
    ReturnTypeError(TypeDef, TypeDef),
    IdentDoesNotExist(String),
    IdentAlreadyExists(String),
    NonFunctionCall,
    WrongNumberOfParametes(usize, usize),
    WrongParamType(TypeDef, TypeDef),
    NonPointerDeref,
    IndexMustBeInteger,
    ConditionMustBeInt,
    InvalidOperation(Operator),
    BinaryTypeMissmatch(Operator, TypeDef, TypeDef),
    BinaryOperatorError,
    CannotAssignInto(Expr),
    TypeParametrMissmatch,
    TypeIsNotSized,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FrontendError {
    Lexer(LexerError),
    Parser(ParserError),
    Type(Vec<TypeError>),
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
        FrontendError::Type(vec![e])
    }
}
