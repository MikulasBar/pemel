use super::token::Token;
use crate::eval_error::EvalError;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedChar(char),
    WrongNumberOfArgs(usize),
    FunctionNotRecognized(String),
    EvalError(EvalError),
}
