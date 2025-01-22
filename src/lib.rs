// #![allow(unused)]
// #![deny(warnings)]

mod eval_error;
mod expr;
mod macros;
mod parser;

#[cfg(test)]
mod expr_test;

pub mod prelude {
    pub use crate::eval_error::EvalError;
    pub use crate::expr::Expr;
    pub use crate::parser::ParseError;
    pub use crate::parser::Token;
}
