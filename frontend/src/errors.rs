#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LexerError {
    UnexpectedCharacter(char),
    UnexpectedEof,
    CharNotClosed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParserError {
    Undefiened,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FrontendError {
    Lexer(LexerError),
    Parser(ParserError),
}
