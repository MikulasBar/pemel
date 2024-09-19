use crate::expr::Expr;

#[test]
fn const_expr() {
    let input = " 8 + 9 * 2  + (2 - 2) + 1";
    let expr = Expr::parse(input);
    let result = expr.eval_const();

    assert_eq!(expr, Expr::Num(27));
    assert_eq!(result, 27);
}


#[test]
fn get_closure_with_var() {
    let input = "7 - x + 8 * (x - 1) - 2";
    let expr = Expr::parse(input);
    let closure = expr.get_closure_with_var("x");

    let result = closure(3);
    assert_eq!(result, 18);
}

#[test]
fn to_string() {
    let input = "7 - x + 8 * (x - 1) - 2";
    let expr = Expr::parse(input);
    let result = expr.to_string();

    print!("{}", result);
}