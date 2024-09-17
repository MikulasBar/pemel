mod lexer;
mod token;
mod parse_error;
mod parser;
mod macros;

use crate::expr::Expr;

pub fn parse(input: &str) -> Expr {
    let tokens = lexer::tokenize(input);
    parser::parse(tokens)
}