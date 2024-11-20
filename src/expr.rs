use core::panic;
use std::fmt::Display;

use crate::macros::expr_pat;
use crate::parser;

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
}

impl Expr {
    pub fn parse(input: &str) -> Result<Expr, parser::ParseError> {
        let tokens = parser::tokenize(input)?;
        parser::parse(tokens)
    }

    pub fn eval_with_variable(&self, var: &str, value: f32) -> f32 {
        match self {
            Expr::Num(n) => *n,
            Expr::Var(s) => if s == var {
                value
            } else {
                panic!("Variable {} is not defined", s)
            },
            
            expr_pat!(BINOP: lhs, rhs) => {
                let lhs = lhs.eval_with_variable(var, value);
                let rhs = rhs.eval_with_variable(var, value);
                self.bin_op_unchecked(lhs, rhs)
            },

            expr_pat!(UNOP: inner) => {
                let inner = inner.eval_with_variable(var, value);
                self.un_op_unchecked(inner)
            }
        }
    }


    pub fn eval_const(&self) -> f32 {
        match self {
            Expr::Num(n) => *n,
            Expr::Var(_) => panic!("Variable found in constant expression"),
            expr_pat!(BINOP: lhs, rhs) => {
                let lhs = lhs.eval_const();
                let rhs = rhs.eval_const();
                self.bin_op_unchecked(lhs, rhs)
            },

            expr_pat!(UNOP: inner) => {
                let inner = inner.eval_const();
                self.un_op_unchecked(inner)
            },
        }
    }

    fn bin_op_unchecked(&self, lhs: f32, rhs: f32) -> f32 {
        match self {
            Expr::Add(_, _) => lhs + rhs,
            Expr::Sub(_, _) => lhs - rhs,
            Expr::Mul(_, _) => lhs * rhs,
            Expr::Div(_, _) => lhs / rhs,
            Expr::Pow(_, _) => lhs.powf(rhs),
            Expr::Log(_, _) => rhs.log(lhs),
            _ => panic!("Not a binary operation: {:?}", self),
        }
    }

    fn un_op_unchecked(&self, inner: f32) -> f32 {
        match self {
            Expr::Sin(_) => inner.sin(),
            Expr::Cos(_) => inner.cos(),
            _ => panic!("Not a unary function: {:?}", self)
        }
    }


    pub fn get_closure_with_var(&self, var: &str) -> Box<dyn Fn(f32) -> f32 + '_> {
        match self {
            Expr::Num(n) => Box::new(|_| *n),
            Expr::Var(s) => if s == var {
                Box::new(|x| x)
            } else {
                panic!("Variable '{}' is not defined", s)
            },

            expr_pat!(BINOP: lhs, rhs) => {
                let lhs = lhs.get_closure_with_var(var);
                let rhs = rhs.get_closure_with_var(var);
                match self {
                    Expr::Add(_, _) => Box::new(move |x| lhs(x) + rhs(x)),
                    Expr::Sub(_, _) => Box::new(move |x| lhs(x) - rhs(x)),
                    Expr::Mul(_, _) => Box::new(move |x| lhs(x) * rhs(x)),
                    Expr::Div(_, _) => Box::new(move |x| lhs(x) / rhs(x)),
                    Expr::Pow(_, _) => Box::new(move |x| lhs(x).powf(rhs(x))),
                    Expr::Log(_, _) => Box::new(move |x| rhs(x).log(lhs(x))),
                    _ => unreachable!(),
                }
            },

            expr_pat!(UNOP: inner) => {
                let inner = inner.get_closure_with_var(var);
                match self {
                    Expr::Sin(_) => Box::new(move |x| inner(x).sin()),
                    Expr::Cos(_) => Box::new(move |x| inner(x).cos()),
                    _ => unreachable!(),
                }
            },
        }
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
            Expr::Num(_) => (),
            Expr::Var(s) if s == var => {
                *self = value.into();
            },
            
            expr_pat!(BINOP: lhs, rhs) => {
                let value = value.into();
                lhs.substitute(var, value.clone());
                rhs.substitute(var, value);
            },
            
            Expr::Sin(x) => x.substitute(var, value),

            _ => (),
        }
    }

    // You need to specify the variable for which you want to take the derivative
    // Before you do that, you need to substitute all other variables with their values
    pub fn aprox_derivative(&self, var: &str) -> Box<dyn Fn(f32, f32) -> f32 + '_> {
        let f = self.get_closure_with_var(var);
        let df = move |x, h: f32| {
            (f(x + h) - f(x)) / h
        };

        Box::new(df)
    }
}

// CONSTRUCTORS
impl Expr {
    pub fn new_mul(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Expr::Mul(
            Box::new(lhs.into()),
            Box::new(rhs.into())
        )
    }

    pub fn new_add(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Expr::Add(
            Box::new(lhs.into()), 
            Box::new(rhs.into())
        )
    }

    pub fn new_sub(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Expr::Sub(
            Box::new(lhs.into()), 
            Box::new(rhs.into())
        )
    }

    pub fn new_div(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Expr::Div(
            Box::new(lhs.into()), 
            Box::new(rhs.into())
        )
    }

    pub fn new_pow(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Expr::Pow(
            Box::new(lhs.into()),
            Box::new(rhs.into()),
        )
    }

    pub fn new_sin(inner: impl Into<Self>) -> Self {
        Expr::Sin(
            Box::new(inner.into())
        )
    }

    pub fn new_cos(inner: impl Into<Self>) -> Self {
        Expr::Cos(
            Box::new(inner.into())
        )
    }
}


impl Display for Expr {
    fn fmt(&self,  f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result  {
        match self {
            Expr::Num(n) => write!(f,"{}", n),
            Expr::Var(s) => write!(f, "{}", s),
            Expr::Log(base, arg) => write!(f, "log_{}({})", base.to_string(), arg.to_string()),

            expr_pat!(BINOP: lhs, rhs) => write!(f, "({} {} {})",
                lhs.to_string(),
                binop_to_string_unchecked(self),
                rhs.to_string()
            ),

            expr_pat!(UNOP: inner) => write!(f, "{}({})",
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
        _ => panic!("Not a binary op")
    }
}

fn unop_to_string_unchecked(expr: &Expr) -> String {
    match expr {
        Expr::Sin(_) => "sin",
        Expr::Cos(_) => "cos",
        _ => panic!("Not a unary op")
    }.to_string()
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
