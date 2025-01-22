macro_rules! expr_pat {
    (BINOP: $lhs:ident, $rhs:ident) => {
        Expr::Add($lhs, $rhs)
            | Expr::Sub($lhs, $rhs)
            | Expr::Mul($lhs, $rhs)
            | Expr::Div($lhs, $rhs)
            | Expr::Pow($lhs, $rhs)
            | Expr::Log($lhs, $rhs)
    };

    (UNOP: $inner:ident) => {
        Expr::Sin($inner) | Expr::Cos($inner)
    };
}

#[allow(unused_braces)]
pub(super) use expr_pat;
