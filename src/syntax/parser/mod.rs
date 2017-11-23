use lalrpop_util;

use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

use syntax::ast::{binary, host, Program};
use source::BytePos;

mod lexer;
#[allow(unused_extern_crates)]
mod grammar;

#[cfg(test)]
mod tests;

use self::lexer::{Error as LexerError, Lexer};

pub type ParseError = lalrpop_util::ParseError<BytePos, String, LexerError>;

fn from_lalrpop_err<L, T: fmt::Debug, E>(
    src: lalrpop_util::ParseError<L, T, E>,
) -> lalrpop_util::ParseError<L, String, E> {
    use lalrpop_util::ParseError::*;

    match src {
        InvalidToken { location } => InvalidToken { location },
        UnrecognizedToken { token, expected } => UnrecognizedToken {
            token: token.map(|(lo, token, hi)| (lo, format!("{:?}", token), hi)),
            expected,
        },
        ExtraToken {
            token: (lo, token, hi),
        } => ExtraToken {
            token: (lo, format!("{:?}", token), hi),
        },
        User { error } => User { error },
    }
}

impl FromStr for Program<String> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Program<String>, ParseError> {
        grammar::parse_Program(Lexer::new(src)).map_err(from_lalrpop_err)
    }
}

impl FromStr for host::Expr<String> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<host::Expr<String>, ParseError> {
        grammar::parse_Expr(Lexer::new(src))
            .map(|expr| Rc::try_unwrap(expr).unwrap())
            .map_err(from_lalrpop_err)
    }
}

impl FromStr for binary::Type<String> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<binary::Type<String>, ParseError> {
        grammar::parse_Type(Lexer::new(src))
            .map(|ty| Rc::try_unwrap(ty).unwrap())
            .map_err(from_lalrpop_err)
    }
}