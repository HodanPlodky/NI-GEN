use ast::Program;
use errors::FrontendError;
use lexer::Lexer;
use parser::Parser;
use typecheck::type_program;

mod ast;
mod errors;
mod lexer;
mod parser;
mod typeast;
mod typecheck;

pub fn parse(input: String, filename: String) -> Result<Program, FrontendError> {
    let lex = Lexer::new(filename, input.chars().peekable());
    let mut parser = Parser::new(lex)?;

    let mut program = parser.parse()?;
    type_program(&mut program)?;
    Ok(program)
}
