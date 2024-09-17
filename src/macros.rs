

macro_rules! expr_pat {
    (BINOP: $lhs:ident, $rhs:ident) => {
          Expr::Add($lhs, $rhs) 
        | Expr::Sub($lhs, $rhs) 
        | Expr::Mul($lhs, $rhs) 
        | Expr::Div($lhs, $rhs)
    };
}

pub(super) use {
    expr_pat,
};
