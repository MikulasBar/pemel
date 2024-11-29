

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    VariableNotDefined(String),
    DivisionByZero,
    InvalidExponentiation,
    InvalidLogarithm,
}