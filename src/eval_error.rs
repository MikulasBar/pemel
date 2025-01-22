/// This represents an error that occurs during evaluation of an expression
/// 
/// `ParseError` can contain this error if the parser tries to evaluate an expression during parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    VariableNotDefined(String),
    DivisionByZero,
    InvalidExponentiation,
    InvalidLogarithm,
}
