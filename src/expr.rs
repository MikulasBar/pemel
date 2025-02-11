use std::fmt::Display;

use crate::eval_error::EvalError;
use crate::macros::expr_pat;
use crate::parser;

/// Represensts a mathematical expression
/// 
/// Expressions are represented as a tree of operations.
/// 
/// ## Parsing
/// 
/// To get this tree from a string, use the `parse` method.
/// 
/// If the parsing fails, the method will return an error.
/// 
/// Parser can evaluate constant parts of the expression during parsing.
/// 
/// This is done to not evaluate constant parts multiple times in the evaluation step.
/// 
/// This means that the parser will return an error if the expression is invalid.
/// 
/// This behavior can be unexpected, so it can be disabled by setting the `implicit_evaluation` parameter to `false`.
/// 
/// ## Evaluation
/// 
/// You can evaluate the expression with 0 or 1 variable using the `eval_const` or `eval_with_variable` method.
/// 
/// These operations can return error if the expression is invalid or if the variable is not defined.
/// 
/// ## Derivative
/// 
/// You can use the `approx_derivative` method to approximate the derivative of the expression with respect to a variable.
/// 
/// This method can't be used for expressions with multiple variables.
/// 
/// There is also a function like derivative, `D(x, ...)`.
/// 
/// First argument is always single variable and the second is the expression.
/// 
/// Function like derivative also can't be used for expressions with multiple variables. (yet)
/// 
/// If you try to use it, it will panic.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f32),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Log(Box<Expr>, Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Cot(Box<Expr>),
    Abs(Box<Expr>),
    Derivative(Box<Expr>, String),
}

impl Default for Expr {
    fn default() -> Self {
        Expr::Num(0.0)
    }
}

impl Expr {
    pub fn parse(input: &str, implicit_evaluation: bool) -> Result<Expr, parser::ParseError> {
        let tokens = parser::tokenize(input)?;
        parser::parse(tokens, implicit_evaluation)
    }

    pub fn eval_with_var(&self, var: &str, value: f32) -> Result<f32, EvalError> {
        match self {
            Expr::Derivative(expr, var) => expr.approx_derivative(var, value, 0.001),
            Expr::Num(n) => Ok(*n),
            Expr::Var(s) => {
                if s == var {
                    Ok(value)
                } else {
                    Err(EvalError::VariableNotDefined(s.clone()))
                }
            }

            expr_pat!(BINOP: lhs, rhs) => {
                let lhs = lhs.eval_with_var(var, value)?;
                let rhs = rhs.eval_with_var(var, value)?;
                self.bin_op_unchecked(lhs, rhs)
            }

            expr_pat!(UNOP: inner) => {
                let inner = inner.eval_with_var(var, value)?;
                self.un_op_unchecked(inner)
            }
        }
    }

    /// Evaluate the expression with the given values for the variables
    /// 
    /// This function is not meant to be used for lot of variables
    /// 
    /// It is O(n) where n is the number of variables
    /// 
    /// Panics for derivative - not implemented yet
    pub fn eval_with(&self, values: &[(&str, f32)]) -> Result<f32, EvalError> {
        match self {
            Expr::Num(n) => Ok(*n),
            Expr::Var(s) => {
                for (var, value) in values {
                    if s == var {
                        return Ok(*value);
                    }
                }

                Err(EvalError::VariableNotDefined(s.clone()))
            }

            Expr::Derivative(..) => {
                todo!("Derivative with multiple variables is not implemented yet")
            }

            expr_pat!(BINOP: lhs, rhs) => {
                let lhs = lhs.eval_with(values)?;
                let rhs = rhs.eval_with(values)?;
                self.bin_op_unchecked(lhs, rhs)
            }

            expr_pat!(UNOP: inner) => {
                let inner = inner.eval_with(values)?;
                self.un_op_unchecked(inner)
            }
        }
    }

    pub fn eval_const(&self) -> Result<f32, EvalError> {
        match self {
            Expr::Derivative(expr, _) => expr.eval_const(),
            Expr::Num(n) => Ok(*n),
            Expr::Var(s) => return Err(EvalError::VariableNotDefined(s.clone())),

            expr_pat!(BINOP: lhs, rhs) => {
                let lhs = lhs.eval_const()?;
                let rhs = rhs.eval_const()?;
                self.bin_op_unchecked(lhs, rhs)
            }

            expr_pat!(UNOP: inner) => {
                let inner = inner.eval_const()?;
                self.un_op_unchecked(inner)
            }
        }
    }

    // This function just checks for the operator but not the operands
    // This can seem unlogical but it enables matching for more than one operator at once
    // (see the eval_const ...)
    fn bin_op_unchecked(&self, lhs: f32, rhs: f32) -> Result<f32, EvalError> {
        Ok(match self {
            Expr::Add(_, _) => lhs + rhs,
            Expr::Sub(_, _) => lhs - rhs,
            Expr::Mul(_, _) => lhs * rhs,
            Expr::Div(_, _) => {
                if rhs == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }

                lhs / rhs
            }

            Expr::Pow(_, _) => {
                if lhs == 0.0 && rhs <= 0.0 {
                    return Err(EvalError::InvalidExponentiation);
                }

                lhs.powf(rhs)
            }

            Expr::Log(_, _) => {
                if lhs <= 0.0 || rhs <= 0.0 {
                    return Err(EvalError::InvalidLogarithm);
                }

                rhs.log(lhs)
            }

            // Panic is safe because we know it's binop
            _ => panic!("Not a binary operation: {:?}", self),
        })
    }

    fn un_op_unchecked(&self, inner: f32) -> Result<f32, EvalError> {
        Ok(match self {
            Expr::Abs(_) => inner.abs(),
            Expr::Sin(_) => inner.sin(),
            Expr::Cos(_) => inner.cos(),
            Expr::Tan(_) => inner.tan(),
            Expr::Cot(_) => {
                let tan = inner.tan();
                if tan == 0.0 {
                    return Err(EvalError::DivisionByZero);
                } else  {
                    1.0 / tan
                }
            },

            // Panic is safe because we know it's binop
            _ => panic!("Not a unary function: {:?}", self),
        })
    }

    // pub fn simplify(&mut self) {
    //     *self = match self {
    //         Expr::Num(_) => self.clone(),
    //         Expr::Var(_) => self.clone(),
    //         Expr::Sub(a, b) if a == b => Expr::Num(0.0),
    //         Expr::Div(a, b) if a == b => Expr::Num(1.0),
    //         // Expr::Sin(e) => Expr::Sin(Box::new(e.simplify())),
    //         _ => return,
    //     }
    // }

    pub fn substitute(&mut self, var: &str, value: impl Into<Expr>) {
        match self {
            Expr::Var(s) if s == var => {
                *self = value.into();
            }

            Expr::Derivative(expr, _) => expr.substitute(var, value),

            expr_pat!(BINOP: lhs, rhs) => {
                let value = value.into();
                lhs.substitute(var, value.clone());
                rhs.substitute(var, value);
            }

            expr_pat!(UNOP: inner) => inner.substitute(var, value),

            Expr::Num(_) => (),
            Expr::Var(_) => (), // I don't want to have the wild card here, because I want to be explicit
        }
    }

    /// Approximate the derivative of the expression with respect to a given variable
    /// 
    /// Only works for expressions with one variable
    pub fn approx_derivative(&self, var: &str, value: f32, h: f32) -> Result<f32, EvalError> {
        let f1 = self.eval_with_var(var, value - h)?;
        let f2 = self.eval_with_var(var, value + h)?;

        Ok((f2 - f1) / (2.0 * h))
    }
}

// CONSTRUCTORS
impl Expr {
    pub fn new_mul(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Expr::Mul(Box::new(lhs.into()), Box::new(rhs.into()))
    }

    pub fn new_add(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Expr::Add(Box::new(lhs.into()), Box::new(rhs.into()))
    }

    pub fn new_sub(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Expr::Sub(Box::new(lhs.into()), Box::new(rhs.into()))
    }

    pub fn new_div(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Expr::Div(Box::new(lhs.into()), Box::new(rhs.into()))
    }

    pub fn new_pow(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Expr::Pow(Box::new(lhs.into()), Box::new(rhs.into()))
    }

    pub fn new_log(base: impl Into<Self>, arg: impl Into<Self>) -> Self {
        Expr::Log(Box::new(base.into()), Box::new(arg.into()))
    }

    pub fn new_sin(inner: impl Into<Self>) -> Self {
        Expr::Sin(Box::new(inner.into()))
    }

    pub fn new_cos(inner: impl Into<Self>) -> Self {
        Expr::Cos(Box::new(inner.into()))
    }

    pub fn new_tan(inner: impl Into<Self>) -> Self {
        Expr::Tan(Box::new(inner.into()))
    }

    pub fn new_cot(inner: impl Into<Self>) -> Self {
        Expr::Cot(Box::new(inner.into()))
    }

    pub fn new_abs(inner: impl Into<Self>) -> Self {
        Expr::Abs(Box::new(inner.into()))
    }

    pub fn new_derivative(var: impl Into<String>, expr: impl Into<Self>) -> Self {
        Expr::Derivative(Box::new(expr.into()), var.into())
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Var(s) => write!(f, "{}", s),
            Expr::Log(base, arg) => write!(f, "log({}, {})", base.to_string(), arg.to_string()),
            Expr::Derivative(expr, var) => write!(f, "D({}, {})", var, expr.to_string()),

            expr_pat!(BINOP: lhs, rhs) => write!(
                f,
                "({} {} {})",
                lhs.to_string(),
                binop_to_string_unchecked(self),
                rhs.to_string()
            ),

            expr_pat!(UNOP: inner) => write!(
                f,
                "{}({})",
                unop_to_string_unchecked(self),
                inner.to_string(),
            ),
        }
    }
}

fn binop_to_string_unchecked(expr: &Expr) -> char {
    match expr {
        Expr::Add(_, _) => '+',
        Expr::Sub(_, _) => '-',
        Expr::Mul(_, _) => '*',
        Expr::Div(_, _) => '/',
        Expr::Pow(_, _) => '^',
        _ => panic!("Not a binary op"),
    }
}

fn unop_to_string_unchecked(expr: &Expr) -> String {
    match expr {
        Expr::Sin(_) => "sin",
        Expr::Cos(_) => "cos",
        Expr::Tan(_) => "tan",
        Expr::Cot(_) => "cot",
        Expr::Abs(_) => "abs",
        _ => panic!("Not a unary op"),
    }
    .to_string()
}

mod froms {
    use super::*;

    impl From<f32> for Expr {
        fn from(n: f32) -> Self {
            Expr::Num(n)
        }
    }

    impl From<&str> for Expr {
        fn from(s: &str) -> Self {
            Expr::Var(s.to_string())
        }
    }

    impl From<String> for Expr {
        fn from(s: String) -> Self {
            Expr::Var(s)
        }
    }
}
