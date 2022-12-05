use logos::Span;

use crate::ast::Identifier;

#[derive(Debug)]
pub struct Spanned<T>(pub T, pub Span);

pub type CacauResult<T = ()> = std::result::Result<T, CacauError>;

#[derive(Debug)]
pub enum CacauError {
    // #[error("unexpected token: {0}")]
    UnexpectedToken(Spanned<()>),
    // #[error("undefined variable: {0}")]
    UndefinedVariable(Identifier),
    // #[error("undefined function: {0}")]
    UndefinedFunction(Identifier),
    // #[error("invalid types for binary operator '{1}': '{0}' and '{2}'")]
    InvalidBinaryOperation(&'static str, &'static str, &'static str),
    // #[error("invalid types for unary operator '{0}': '{1}'")]
    InvalidUnaryOperation(&'static str, &'static str),
    // #[error("function '{0}' expected {1} arguments, but received {2}")]
    WrongAmountOfArguments(Identifier, usize, usize),
    // #[error("function '{0}' expected argument of type {1}, received {2} instead")]
    FunctionWrongArgumentType(&'static str, &'static str, &'static str),
    // #[error("assertion error")]
    AssertionError,
}
