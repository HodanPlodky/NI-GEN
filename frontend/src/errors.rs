#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LexerError {
    UnexpectedCharacter(char),
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
