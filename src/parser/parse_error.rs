use super::token::Token;
use crate::eval_error::EvalError;

/// Error indicating that the parser encountered an unexpected syntax
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedChar(char),
    WrongNumberOfArgs(usize),
    FunctionNotRecognized(String),

    /// This error can occur when the parser is evaluating during parsing.
    EvalError(EvalError),
}
