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
    Sin(Box<Expr>),
}

impl Expr {
    pub fn parse(input: &str) -> Expr {
        let tokens = parser::tokenize(input);
        parser::parse(tokens)
    }

    pub fn num_unwrap(&self) -> f32 {
        match self {
            Expr::Num(n) => *n,
            _ => panic!("Not a number"),
        }
    }

    fn bin_op_unchecked(&self, lhs: f32, rhs: f32) -> f32 {
        match self {
            Expr::Add(_, _) => lhs + rhs,
            Expr::Sub(_, _) => lhs - rhs,
            Expr::Mul(_, _) => lhs * rhs,
            Expr::Div(_, _) => lhs / rhs,
            Expr::Pow(_, _) => lhs.powf(rhs),
            _ => panic!("Not a binary operation"),
        }
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

            Expr::Sin(x) => x.eval_with_variable(var, value).sin(),
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
            Expr::Sin(x) => x.eval_const().sin(),
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
                    _ => unreachable!(),
                }
            },
            Expr::Sin(x) => {
                let x = x.get_closure_with_var(var);
                Box::new(move |x| x.sin())
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

    // This returns a closure that takes a value and a step size h like in definition of derivative
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



impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Num(n) => n.to_string(),
            Expr::Var(s) => s.clone(),
            expr_pat!(BINOP: lhs, rhs) => format!("({} {} {})",
                lhs.to_string(),
                bin_op_to_char_unchecked(self),
                rhs.to_string()
            ),
            Expr::Sin(x) => format!("sin{}", x.to_string()),
        }
    }
}


fn bin_op_to_char_unchecked(expr: &Expr) -> char {
    match expr {
        Expr::Add(_, _) => '+',
        Expr::Sub(_, _) => '-',
        Expr::Mul(_, _) => '*',
        Expr::Div(_, _) => '/',
        Expr::Pow(_, _) => '^',
        _ => panic!("Not a binary operation"),
    }
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
