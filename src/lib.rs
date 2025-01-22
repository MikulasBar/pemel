// #![allow(unused)]
// #![deny(warnings)]

mod expr;
mod parser;
mod eval_error;
mod macros;

#[cfg(test)]
mod expr_test;

pub mod prelude {
    pub use crate::expr::Expr;
    pub use crate::eval_error::EvalError;
    pub use crate::parser::ParseError;
    pub use crate::parser::Token;
}

