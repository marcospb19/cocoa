#![feature(box_syntax)]

mod ast;
mod error;
mod lexer;

pub use lalrpop_cacau::cacau_parser::ProgramParser;
pub use lalrpop_util::ParseError;

pub use self::{
    ast::{Interpret, Statement, Vm},
    error::{CacauError, CacauResult, Spanned},
    lexer::{Lexer, Tok},
};

#[allow(warnings, unused)]
mod lalrpop_cacau {
    use lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub cacau_parser);
}

pub fn parse_input(input: &str) -> Result<Vec<Statement>, ParseError<usize, Tok<'_>, CacauError>> {
    let lexer = Lexer::new(input);
    ProgramParser::new().parse(input, lexer)
}

pub fn interpret_ast(statements: &[Statement]) -> CacauResult {
    let mut vm = Vm::new();

    for statement in statements {
        dbg!(&statement);
        statement.evaluate(&mut vm)?;
    }
    Ok(())
}
