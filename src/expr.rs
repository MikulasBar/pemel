use crate::macros::expr_pat;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i32),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    // Sin(Box<Expr>),
}

impl Expr {
    pub fn num_unwrap(&self) -> i32 {
        match self {
            Expr::Num(n) => *n,
            _ => panic!("Not a number"),
        }
    }

    fn bin_op_unchecked(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            Expr::Add(_, _) => lhs + rhs,
            Expr::Sub(_, _) => lhs - rhs,
            Expr::Mul(_, _) => lhs * rhs,
            Expr::Div(_, _) => lhs / rhs,
            _ => panic!("Not a binary operation"),
        }
    }

    pub fn eval_with_variable(&self, var: &str, value: i32) -> i32 {
        match self {
            Expr::Num(n) => *n,
            Expr::Var(s) => if s == var { value } else { 0 },
            expr_pat!(BINOP: lhs, rhs) => {
                let lhs = lhs.eval_with_variable(var, value);
                let rhs = rhs.eval_with_variable(var, value);
                self.bin_op_unchecked(lhs, rhs)
            },
            // Expr::Sin(e) => (e.eval_with_var(var, val) as f64).sin() as i32,
        }
    }

    pub fn eval_const(&self) -> i32 {
        match self {
            Expr::Num(n) => *n,
            Expr::Var(_) => panic!("Variable found in constant expression"),
            expr_pat!(BINOP: lhs, rhs) => {
                let lhs = lhs.eval_const();
                let rhs = rhs.eval_const();
                self.bin_op_unchecked(lhs, rhs)
            },
            // Expr::Sin(e) => (e.eval_constant() as f64).sin() as i32,
        }
    }

    pub fn simplify(&mut self) {
        *self = match self {
            Expr::Num(_) => self.clone(),
            Expr::Var(_) => self.clone(),
            Expr::Sub(a, b) if a == b => Expr::Num(0),
            Expr::Div(a, b) if a == b => Expr::Num(1),
            // Expr::Sin(e) => Expr::Sin(Box::new(e.simplify())),
            _ => return,
        }
    }
}