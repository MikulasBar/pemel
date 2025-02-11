use super::token::Token;
use crate::eval_error::EvalError;

/// Error indicating that the parser encountered an unexpected syntax
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedChar(char),
    WrongNumberOfArgs(usize),
    FunctionNotRecognized(String),
    /// Indicates that a derivative was taken with repsect to a non-variable
    DerivativeNotVariable(String),

    /// This error can occur when the parser is evaluating during parsing.
    EvalError(EvalError),
}
