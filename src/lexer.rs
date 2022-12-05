use logos::Logos;

use crate::{CacauError, Spanned};

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
pub enum Tok<'input> {
    #[regex(r"[_a-zA-Z][a-zA-Z0-9_]*")]
    Identifier(&'input str),
    #[regex(r"[0-9][0-9_]*")]
    Number(&'input str),
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("not")]
    Not,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token(";")]
    SemiColon,
    #[token("=")]
    SingleEquals,
    #[token(",")]
    Comma,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBracket,
    #[token("}")]
    CloseBracket,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("<")]
    Less,
    #[token("<=")]
    LessOrEqual,
    #[token("==")]
    DoubleEquals,
    #[token("!=")]
    NotEquals,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterOrEqual,

    #[error]
    #[regex(r"\s+", logos::skip)]
    #[regex(r"//.*\n", logos::skip)]
    Invalid,
}

pub struct Lexer<'a> {
    lex: logos::SpannedIter<'a, Tok<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            lex: Tok::lexer(src).spanned(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(usize, Tok<'a>, usize), CacauError>;

    fn next(&mut self) -> Option<Self::Item> {
        let (tok, span) = self.lex.next()?;

        if tok == Tok::Invalid {
            dbg!("here");
            Some(Err(CacauError::UnexpectedToken(Spanned((), span))))
        } else {
            Some(Ok((span.start, tok, span.end)))
        }
    }
}
